use std::fmt::Display;

pub enum Object {
    INTEGER(i64),
    BOOLEAN(bool),
    NULL,
}

impl Display for Object{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::INTEGER(i) => write!(f, "{}", i),
            Object::BOOLEAN(b) => write!(f, "{}", b),
            Object::NULL => write!(f, "null"),
        }
    }

}
