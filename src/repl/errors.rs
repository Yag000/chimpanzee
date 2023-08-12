use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct LexerErrors {
    errors: Vec<String>,
}

impl Display for LexerErrors {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "Lexer errors:")?;
        for err in &self.errors {
            writeln!(f, "\t{err}")?;
        }
        Ok(())
    }
}
impl Default for LexerErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl LexerErrors {
    pub fn new() -> LexerErrors {
        LexerErrors { errors: vec![] }
    }

    pub fn add_error(&mut self, err: String) {
        self.errors.push(err);
    }

    pub fn add_errors(&mut self, mut errs: LexerErrors) {
        self.errors.append(&mut errs.errors);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl Error for LexerErrors {}

#[derive(Debug)]
pub struct CompilerError {
    error: String,
}

impl CompilerError {
    pub fn new(error: String) -> CompilerError {
        CompilerError { error }
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "Compiler error:\n\t{}", self.error)
    }
}

impl Error for CompilerError {}

#[derive(Debug)]
pub struct RuntimeError {
    error: String,
}

impl RuntimeError {
    pub fn new(error: String) -> RuntimeError {
        RuntimeError { error }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "Runtime error:\n\t{}", self.error)
    }
}

impl Error for RuntimeError {}
