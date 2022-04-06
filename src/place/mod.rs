use cubob::display_list_from_iter;
use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    iter::FusedIterator,
};

#[derive(Clone, PartialEq, Eq)]
pub struct CodePlace {
    file: &'static str,
    line: u32,
    column: u32,
}

impl CodePlace {
    pub const fn new(file: &'static str, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }
}

impl Display for CodePlace {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

impl Debug for CodePlace {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self, f)
    }
}

#[macro_export]
macro_rules! place {
    () => {
        $crate::CodePlace::new(file!(), line!(), column!())
    };
}

#[derive(Clone, PartialEq, Eq)]
pub struct CodePlaceChain {
    head: CodePlace,
    tail: Option<Box<CodePlaceChain>>,
}

impl CodePlaceChain {
    pub fn prepend(self, place: CodePlace) -> Self {
        Self {
            head: place,
            tail: Some(Box::new(self)),
        }
    }

    pub fn prepend_mut(&mut self, place: CodePlace) -> &mut Self {
        let mut new_node = Self {
            head: place,
            tail: None,
        };
        std::mem::swap(self, &mut new_node);
        self.tail = Some(Box::new(new_node));
        self
    }
}

impl From<CodePlace> for CodePlaceChain {
    fn from(src: CodePlace) -> Self {
        Self {
            head: src,
            tail: None,
        }
    }
}

impl<'a> IntoIterator for &'a CodePlaceChain {
    type Item = &'a CodePlace;
    type IntoIter = CodePlaceChainIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CodePlaceChainIter(Some(self))
    }
}

pub struct CodePlaceChainIter<'a>(Option<&'a CodePlaceChain>);

impl<'a> Iterator for CodePlaceChainIter<'a> {
    type Item = &'a CodePlace;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            None => None,
            Some(entry) => {
                let current = &entry.head;
                self.0 = entry.tail.as_deref();
                Some(current)
            }
        }
    }
}

impl FusedIterator for CodePlaceChainIter<'_> {}

impl Display for CodePlaceChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        display_list_from_iter(f, self.into_iter())
    }
}

impl Debug for CodePlaceChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests;
