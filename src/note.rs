use std::{
    borrow::{Borrow, Cow},
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::Deref,
};

pub type StaticCowStr = Cow<'static, str>;

fn opt_text<T: AsRef<str>>(val: T) -> Option<T> {
    match val.as_ref().is_empty() {
        true => None,
        false => Some(val),
    }
}

#[derive(Clone, Eq)]
pub struct Note(Option<StaticCowStr>);

impl Note {
    pub const NONE: Self = Self(None);

    pub fn text(&self) -> Option<&str> {
        self.0.as_ref().map(Borrow::borrow)
    }
}

impl From<StaticCowStr> for Note {
    fn from(src: StaticCowStr) -> Self {
        Self(opt_text(src))
    }
}

impl From<String> for Note {
    fn from(src: String) -> Self {
        Self(opt_text(src).map(StaticCowStr::Owned))
    }
}

impl From<&'static str> for Note {
    fn from(src: &'static str) -> Self {
        Self(opt_text(src).map(StaticCowStr::Borrowed))
    }
}

impl<S: Into<StaticCowStr> + AsRef<str>> From<Option<S>> for Note {
    fn from(src: Option<S>) -> Self {
        Self(src.and_then(opt_text).map(Into::into))
    }
}

impl Deref for Note {
    type Target = Option<StaticCowStr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            None => write!(f, "``"),
            Some(text) => Display::fmt(&text, f),
        }
    }
}

impl Debug for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (Some(t1), Some(t2)) => t1.as_ref() == t2.as_ref(),
            _ => false,
        }
    }
}

impl PartialEq<&str> for Note {
    fn eq(&self, other: &&str) -> bool {
        match self.0 {
            None => other.is_empty(),
            Some(ref text) => text == other,
        }
    }
}

#[macro_export]
macro_rules! note {
    () => {
        $crate::Note::NONE
    };

    ($text:expr) => {
        $crate::Note::from($text)
    };

    ($format:expr, $($rest:tt)*) => {
        $crate::Note::from(format!($format, $($rest)*))
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn primitive() {
        assert_eq!(note!(), Note::NONE);
        assert_eq!(note!("Just some static text"), "Just some static text");
        assert_eq!(
            note!("Just some dynamic text".to_owned()),
            "Just some dynamic text"
        );
    }
}
