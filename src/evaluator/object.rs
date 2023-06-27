use std::{cell::RefCell, cmp::Ordering, fmt::Display, rc::Rc};

use crate::parser::ast::{BlockStatement, Identifier};

use super::{enviroment::Environment, NULL};
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    INTEGER(i64),
    BOOLEAN(bool),
    STRING(String),
    RETURN(Box<Object>),
    ERROR(String),
    FUNCTION(FunctionObject),
    BUILTIN(BuiltinFunction),
    ARRAY(Vec<Object>),
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
            Object::BUILTIN(o) => write!(f, "{o}"),
            Object::ERROR(s) => write!(f, "ERROR: {s}"),
            Object::ARRAY(a) => Self::format_array(f, a),
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
            Object::BUILTIN(_) => String::from("BUILTIN"),
            Object::ARRAY(_) => String::from("ARRAY"),
            Object::NULL => String::from("NULL"),
        }
    }

    fn format_array(f: &mut std::fmt::Formatter<'_>, array: &[Object]) -> std::fmt::Result {
        let values: Vec<String> = array.iter().map(|o| o.to_string()).collect();
        write!(f, "[{}]", values.join(", "))
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

#[derive(Debug, PartialEq, Clone)]
pub enum BuiltinFunction {
    LEN,
    FIRST,
    LAST,
    REST,
    PUSH,
}

impl Display for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinFunction::LEN => write!(f, "len"),
            BuiltinFunction::FIRST => write!(f, "first"),
            BuiltinFunction::LAST => write!(f, "last"),
            BuiltinFunction::REST => write!(f, "rest"),
            BuiltinFunction::PUSH => write!(f, "push"),
        }
    }
}

impl BuiltinFunction {
    pub fn call(&self, args: Vec<Object>) -> Object {
        match self {
            BuiltinFunction::LEN => self.call_len(args),
            BuiltinFunction::FIRST => self.call_first(args),
            BuiltinFunction::LAST => self.call_last(args),
            BuiltinFunction::REST => self.call_rest(args),
            BuiltinFunction::PUSH => self.call_push(args),
        }
    }

    pub fn get_builtins() -> Environment {
        let mut env = Environment::new();
        env.set(String::from("len"), Object::BUILTIN(BuiltinFunction::LEN));
        env.set(
            String::from("first"),
            Object::BUILTIN(BuiltinFunction::FIRST),
        );
        env.set(String::from("last"), Object::BUILTIN(BuiltinFunction::LAST));
        env.set(String::from("rest"), Object::BUILTIN(BuiltinFunction::REST));
        env.set(String::from("push"), Object::BUILTIN(BuiltinFunction::PUSH));
        env
    }

    pub fn get_builtin(name: &str) -> Option<Object> {
        match name {
            "len" => Some(Object::BUILTIN(BuiltinFunction::LEN)),
            "first" => Some(Object::BUILTIN(BuiltinFunction::FIRST)),
            "last" => Some(Object::BUILTIN(BuiltinFunction::LAST)),
            "rest" => Some(Object::BUILTIN(BuiltinFunction::REST)),
            "push" => Some(Object::BUILTIN(BuiltinFunction::PUSH)),
            _ => None,
        }
    }

    fn call_len(&self, args: Vec<Object>) -> Object {
        Self::handle_number_of_arguments(args.len(), 1).unwrap_or_else(|| match &args[0] {
            Object::STRING(s) => Object::INTEGER(s.len() as i64),
            Object::ARRAY(a) => Object::INTEGER(a.len() as i64),
            _ => Object::ERROR(format!(
                "argument to `len` not supported, got {}",
                args[0].get_type()
            )),
        })
    }

    fn call_first(&self, args: Vec<Object>) -> Object {
        Self::handle_number_of_arguments(args.len(), 1).unwrap_or_else(|| match &args[0] {
            Object::ARRAY(a) => {
                if !a.is_empty() {
                    a[0].clone()
                } else {
                    NULL
                }
            }
            _ => Object::ERROR(format!(
                "argument to `first` not supported, must be ARRAY, got {}",
                args[0].get_type()
            )),
        })
    }

    fn call_last(&self, args: Vec<Object>) -> Object {
        Self::handle_number_of_arguments(args.len(), 1).unwrap_or_else(|| match &args[0] {
            Object::ARRAY(a) => {
                let length = a.len();
                if length > 0 {
                    a[length - 1].clone()
                } else {
                    NULL
                }
            }
            _ => Object::ERROR(format!(
                "argument to `last` not supported, must be ARRAY, got {}",
                args[0].get_type()
            )),
        })
    }

    fn call_rest(&self, args: Vec<Object>) -> Object {
        Self::handle_number_of_arguments(args.len(), 1).unwrap_or_else(|| match &args[0] {
            Object::ARRAY(a) => {
                let length = a.len();

                match length.cmp(&1) {
                    Ordering::Greater => Object::ARRAY(a[1..length].to_vec()),
                    Ordering::Equal => Object::ARRAY(vec![]),
                    Ordering::Less => NULL,
                }
            }
            _ => Object::ERROR(format!(
                "argument to `rest` not supported, must be ARRAY, got {}",
                args[0].get_type()
            )),
        })
    }

    fn call_push(&self, args: Vec<Object>) -> Object {
        Self::handle_number_of_arguments(args.len(), 2).unwrap_or_else(|| match &args[0] {
            Object::ARRAY(a) => {
                let mut new_array = a.clone();
                new_array.push(args[1].clone());
                Object::ARRAY(new_array)
            }
            _ => Object::ERROR(format!(
                "argument to `push` not supported, must be ARRAY, got {}",
                args[0].get_type()
            )),
        })
    }

    fn handle_number_of_arguments(got: usize, expected: usize) -> Option<Object> {
        if got != expected {
            return Some(Object::ERROR(format!(
                "wrong number of arguments. got={}, want={}",
                got, expected
            )));
        }
        None
    }
}
