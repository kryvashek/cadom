use cubob::{Alternate, StructShow};
use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    iter::FusedIterator,
    ops::Deref,
};

use crate::{CodePlace, CodePlaceChain, Note};

#[derive(Clone, PartialEq, Eq)]
pub enum Decay<O: StdError> {
    Internal {
        note: Note,
        place: CodePlaceChain,
    },
    External {
        error: O,
    },
    Further {
        note: Note,
        place: CodePlaceChain,
        error: Box<Decay<O>>,
    },
}

impl<O: StdError> Decay<O> {
    pub fn new<N: Into<Note>>(place: CodePlace, note: N) -> Self {
        Self::Internal {
            note: note.into(),
            place: place.into(),
        }
    }
    pub fn new_unnoted(place: CodePlace) -> Self {
        Self::Internal {
            note: Note::NONE,
            place: place.into(),
        }
    }

    pub fn further<N: Into<Note>>(mut self, new_place: CodePlace, note: N) -> Self {
        let note = note.into();
        match (note.is_none(), &mut self) {
            (true, Decay::Internal { place, .. } | Decay::Further { place, .. }) => {
                place.prepend_mut(new_place);
                self
            }
            _ => Decay::Further {
                place: new_place.into(),
                note,
                error: Box::new(self),
            },
        }
    }

    pub fn further_unnoted(mut self, new_place: CodePlace) -> Self {
        match &mut self {
            Decay::Internal { place, .. } | Decay::Further { place, .. } => {
                place.prepend_mut(new_place);
                self
            }
            Decay::External { .. } => Decay::Further {
                place: new_place.into(),
                note: Note::NONE,
                error: Box::new(self),
            },
        }
    }

    pub fn morph<N: Into<Note>, E: IntoDecay<O, L>, const L: usize>(
        new_place: CodePlace,
        note: N,
    ) -> impl FnOnce(E) -> Self {
        |error: E| error.into_decay().further(new_place, note)
    }

    pub fn morph_unnoted<E: IntoDecay<O, L>, const L: usize>(
        new_place: CodePlace,
    ) -> impl FnOnce(E) -> Self {
        |error: E| error.into_decay().further_unnoted(new_place)
    }

    pub fn root(&self) -> DecayRoot<'_, O> {
        let mut current = self;
        loop {
            match current {
                Self::Further { ref error, .. } => current = error.deref(),
                Self::Internal {
                    ref note,
                    ref place,
                } => return DecayRoot::Internal { note, place },
                Self::External { ref error } => return DecayRoot::External { error },
            }
        }
    }
}

impl<O: StdError> From<O> for Decay<O> {
    fn from(error: O) -> Self {
        Decay::External { error }
    }
}

pub trait IntoDecay<O: StdError, const L: usize> {
    fn into_decay(self) -> Decay<O>;
}

impl<O: StdError> IntoDecay<O, 0> for Decay<O> {
    fn into_decay(self) -> Decay<O> {
        self
    }
}

impl<E: Into<O>, O: StdError> IntoDecay<O, 1> for E {
    fn into_decay(self) -> Decay<O> {
        self.into().into()
    }
}

impl<O: StdError> Display for Decay<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut output = StructShow::new(f, Alternate::Inherit);
        self.into_iter().for_each(|decay| match decay {
            Decay::External { error } => {
                output.field(&"error", error);
            }
            Decay::Internal { note, .. } | Decay::Further { note, .. } => {
                output.field_opt(&"note", &note.text());
            }
        });
        output.finish()
    }
}

impl<O: StdError> Debug for Decay<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut output = StructShow::new(f, Alternate::Inherit);
        self.into_iter().for_each(|decay| match decay {
            Decay::External { error } => {
                output.field(&"error", error);
            }
            Decay::Internal { note, place } | Decay::Further { note, place, .. } => {
                output
                    .field_override(&"place", place, Alternate::OneLine)
                    .field_opt(&"note", &note.text());
            }
        });
        output.finish()
    }
}

impl<'a, O: StdError> IntoIterator for &'a Decay<O> {
    type Item = &'a Decay<O>;
    type IntoIter = DecayIter<'a, O>;

    fn into_iter(self) -> Self::IntoIter {
        DecayIter(Some(self))
    }
}

pub struct DecayIter<'a, O: StdError>(Option<&'a Decay<O>>);

impl<'a, O: StdError> Iterator for DecayIter<'a, O> {
    type Item = &'a Decay<O>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Some(current @ Decay::Further { error, .. }) => {
                self.0 = Some(error);
                Some(current)
            }
            Some(current) => {
                self.0 = None;
                Some(current)
            }
            None => None,
        }
    }
}

impl<O: StdError> FusedIterator for DecayIter<'_, O> {}

impl<O: StdError + 'static> StdError for Decay<O> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Decay::Internal { .. } => None,
            Decay::External { error } => Some(error),
            Decay::Further { error, .. } => Some(error),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecayRoot<'a, O: StdError> {
    Internal {
        note: &'a Note,
        place: &'a CodePlaceChain,
    },
    External {
        error: &'a O,
    },
}

#[macro_export]
macro_rules! decay {
    () => {
        $crate::Decay::new_unnoted($crate::place!())
    };

    ($text:expr) => {
        $crate::Decay::new($crate::place!(), $text)
    };

    ($format:expr, $($rest:tt)*) => {
        $crate::Decay::new($crate::place!(), format!($format, $($rest)*))
    };
}

#[macro_export]
macro_rules! rot {
    () => {
        $crate::Decay::morph_unnoted($crate::place!())
    };

    ($text:expr) => {
        $crate::Decay::morph($crate::place!(), $text)
    };

    ($format:expr, $($rest:tt)*) => {
        $crate::Decay::morph($crate::place!(), format!($format, $($rest)*))
    };
}

#[cfg(test)]
mod tests;
