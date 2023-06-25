use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Object {
    INTEGER(i64),
    BOOLEAN(bool),
    RETURN(Box<Object>),
    ERROR(String),
    NULL,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::INTEGER(i) => write!(f, "{}", i),
            Object::BOOLEAN(b) => write!(f, "{}", b),
            Object::RETURN(o) => write!(f, "{}", o),
            Object::ERROR(s) => write!(f, "ERROR: {}", s),
            Object::NULL => write!(f, "null"),
        }
    }
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::INTEGER(_) => String::from("INTEGER"),
            Object::BOOLEAN(_) => String::from("BOOLEAN"),
            Object::RETURN(_) => String::from("RETURN"),
            Object::ERROR(_) => String::from("ERROR"),
            Object::NULL => String::from("NULL"),
        }
    }
}
