use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::parser::ast::{BlockStatement, Identifier};

use super::enviroment::Environment;
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    INTEGER(i64),
    BOOLEAN(bool),
    STRING(String),
    RETURN(Box<Object>),
    ERROR(String),
    FUNCTION(FunctionObject),
    NULL,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::INTEGER(i) => write!(f, "{}", i),
            Object::BOOLEAN(b) => write!(f, "{}", b),
            Object::STRING(s) => write!(f, "\"{}\"", s),
            Object::RETURN(o) => write!(f, "{}", o),
            Object::FUNCTION(o) => write!(f, "{}", o),
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
            Object::STRING(_) => String::from("STRING"),
            Object::RETURN(_) => String::from("RETURN"),
            Object::ERROR(_) => String::from("ERROR"),
            Object::FUNCTION(_) => String::from("FUNCTION"),
            Object::NULL => String::from("NULL"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionObject {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub environment: Rc<RefCell<Environment>>,
}

impl Display for FunctionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parameters = self
            .parameters
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        write!(f, "fn({}){{\n{}\n}}", parameters.join(", "), self.body)
    }
}
