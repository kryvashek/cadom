use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

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

#[cfg(test)]
mod tests;
