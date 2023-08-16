use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct ParserErrors {
    pub errors: Vec<String>,
}

impl Error for ParserErrors {}

impl Default for ParserErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ParserErrors {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "Parser errors:")?;
        for err in &self.errors {
            writeln!(f, "\t{err}")?;
        }
        Ok(())
    }
}

impl ParserErrors {
    pub fn new() -> ParserErrors {
        ParserErrors { errors: vec![] }
    }

    pub fn add_error(&mut self, err: String) {
        self.errors.push(err);
    }

    pub fn add_errors(&mut self, mut errors: Vec<String>) {
        self.errors.append(&mut errors);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }
}
