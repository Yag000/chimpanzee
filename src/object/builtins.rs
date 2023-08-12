use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::{cmp::Ordering, fmt::Display};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::object::object::{Object, NULL};

#[derive(Debug, PartialEq, Clone, FromPrimitive, ToPrimitive, EnumIter)]
pub enum BuiltinFunction {
    LEN,
    FIRST,
    LAST,
    REST,
    PUSH,
    PUTS,
}

impl Display for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinFunction::LEN => write!(f, "len"),
            BuiltinFunction::FIRST => write!(f, "first"),
            BuiltinFunction::LAST => write!(f, "last"),
            BuiltinFunction::REST => write!(f, "rest"),
            BuiltinFunction::PUSH => write!(f, "push"),
            BuiltinFunction::PUTS => write!(f, "puts"),
        }
    }
}

#[allow(clippy::needless_pass_by_value)] // false positive
impl BuiltinFunction {
    pub fn get_builtin(name: &str) -> Option<Object> {
        match name {
            "len" => Some(Object::BUILTIN(BuiltinFunction::LEN)),
            "first" => Some(Object::BUILTIN(BuiltinFunction::FIRST)),
            "last" => Some(Object::BUILTIN(BuiltinFunction::LAST)),
            "rest" => Some(Object::BUILTIN(BuiltinFunction::REST)),
            "push" => Some(Object::BUILTIN(BuiltinFunction::PUSH)),
            "puts" => Some(Object::BUILTIN(BuiltinFunction::PUTS)),
            _ => None,
        }
    }

    pub fn get_builtin_by_id(id: usize) -> Option<Object> {
        BuiltinFunction::from_usize(id).map(Object::BUILTIN)
    }

    pub fn get_builtins_names() -> Vec<String> {
        BuiltinFunction::iter().map(|f| f.to_string()).collect()
    }

    pub fn call(&self, args: Vec<Object>) -> Object {
        match self {
            BuiltinFunction::LEN => Self::call_len(args),
            BuiltinFunction::FIRST => Self::call_first(args),
            BuiltinFunction::LAST => Self::call_last(args),
            BuiltinFunction::REST => Self::call_rest(args),
            BuiltinFunction::PUSH => Self::call_push(args),
            BuiltinFunction::PUTS => Self::call_puts(args),
        }
    }

    fn call_len(args: Vec<Object>) -> Object {
        Self::handle_number_of_arguments(args.len(), 1).unwrap_or_else(|| match &args[0] {
            Object::STRING(s) => Object::INTEGER(s.len() as i64),
            Object::ARRAY(a) => Object::INTEGER(a.len() as i64),
            _ => Object::ERROR(format!(
                "argument to `len` not supported, got {}",
                args[0].get_type()
            )),
        })
    }

    fn call_first(args: Vec<Object>) -> Object {
        Self::handle_number_of_arguments(args.len(), 1).unwrap_or_else(|| match &args[0] {
            Object::ARRAY(a) => {
                if a.is_empty() {
                    NULL
                } else {
                    a[0].clone()
                }
            }
            _ => Object::ERROR(format!(
                "argument to `first` not supported, must be ARRAY, got {}",
                args[0].get_type()
            )),
        })
    }

    fn call_last(args: Vec<Object>) -> Object {
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

    fn call_rest(args: Vec<Object>) -> Object {
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

    fn call_push(args: Vec<Object>) -> Object {
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

    fn call_puts(args: Vec<Object>) -> Object {
        for arg in args {
            println!("{arg}");
        }
        NULL
    }

    fn handle_number_of_arguments(got: usize, expected: usize) -> Option<Object> {
        if got != expected {
            return Some(Object::ERROR(format!(
                "wrong number of arguments. got={got}, want={expected}"
            )));
        }
        None
    }
}
