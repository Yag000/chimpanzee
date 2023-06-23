use std::fmt;

type Result<T> = std::result::Result<T, ParsingError>;

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "double")
    }
}
