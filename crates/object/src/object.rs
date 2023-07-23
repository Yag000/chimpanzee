use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    hash::Hash,
    rc::Rc,
};

use parser::ast::{BlockStatement, Identifier};

use crate::enviroment::Environment;

pub const TRUE: Object = Object::BOOLEAN(true);
pub const FALSE: Object = Object::BOOLEAN(false);
pub const NULL: Object = Object::NULL;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    INTEGER(i64),
    BOOLEAN(bool),
    STRING(String),
    RETURN(Box<Object>),
    ERROR(String),
    FUNCTION(Function),
    COMPILEDFUNCTION(CompiledFunction),
    BUILTIN(BuiltinFunction),
    ARRAY(Vec<Object>),
    HASHMAP(HashMap<Object, Object>),
    NULL,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::INTEGER(i) => write!(f, "{i}"),
            Object::BOOLEAN(b) => write!(f, "{b}"),
            Object::STRING(s) => write!(f, "\"{s}\""),
            Object::RETURN(o) => write!(f, "{o}",),
            Object::FUNCTION(o) => write!(f, "{o}"),
            Object::COMPILEDFUNCTION(o) => write!(f, "{o}"),
            Object::BUILTIN(o) => write!(f, "{o}"),
            Object::ERROR(s) => write!(f, "ERROR: {s}"),
            Object::ARRAY(a) => Self::format_array(f, a),
            Object::HASHMAP(h) => {
                let mut values: Vec<String> = h.iter().map(|(k, v)| format!("{k}: {v}")).collect();
                values.sort();
                write!(f, "{{{}}}", values.join(", "))
            }
            Object::NULL => write!(f, "null"),
        }
    }
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Object::INTEGER(i) => i.hash(state),
            Object::BOOLEAN(b) => b.hash(state),
            Object::STRING(s) => s.hash(state),
            _ => "".hash(state),
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
            Object::COMPILEDFUNCTION(_) => String::from("COMPILEDFUNCTION"),
            Object::BUILTIN(_) => String::from("BUILTIN"),
            Object::ARRAY(_) => String::from("ARRAY"),
            Object::HASHMAP(_) => String::from("HASHMAP"),
            Object::NULL => String::from("NULL"),
        }
    }

    fn format_array(f: &mut std::fmt::Formatter<'_>, array: &[Object]) -> std::fmt::Result {
        let values: Vec<String> = array.iter().map(ToString::to_string).collect();
        write!(f, "[{}]", values.join(", "))
    }

    pub fn is_hashable(&self) -> bool {
        matches!(
            self,
            Object::INTEGER(_) | Object::BOOLEAN(_) | Object::STRING(_)
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub environment: Rc<RefCell<Environment>>,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parameters = self
            .parameters
            .iter()
            .map(ToString::to_string)
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

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledFunction {
    pub instructions: Vec<u8>,
}

impl Display for CompiledFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "CompiledFunction({:p})", self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hashing_objects() {
        let mut map = HashMap::new();
        let one = Object::INTEGER(1);
        let two = Object::INTEGER(2);
        let one_again = Object::INTEGER(1);
        let string_1 = Object::STRING("one".to_string());
        let string_2 = Object::STRING("two".to_string());
        let string_1_again = Object::STRING("one".to_string());
        let true_1 = Object::BOOLEAN(true);
        let false_1 = Object::BOOLEAN(false);
        let true_2 = Object::BOOLEAN(true);

        map.insert(one.clone(), "one".to_string());
        map.insert(two.clone(), "two".to_string());
        map.insert(one_again.clone(), "one again".to_string());
        map.insert(string_1.clone(), "one".to_string());
        map.insert(string_2.clone(), "two".to_string());
        map.insert(string_1_again.clone(), "one again".to_string());
        map.insert(true_1.clone(), "true".to_string());
        map.insert(false_1.clone(), "false".to_string());
        map.insert(true_2.clone(), "true again".to_string());

        assert_eq!(map.len(), 6);
        assert_eq!(map.get(&one), Some(&"one again".to_string()));
        assert_eq!(map.get(&two), Some(&"two".to_string()));
        assert_eq!(map.get(&one_again), Some(&"one again".to_string()));
        assert_eq!(map.get(&string_1), Some(&"one again".to_string()));
        assert_eq!(map.get(&string_2), Some(&"two".to_string()));
        assert_eq!(map.get(&string_1_again), Some(&"one again".to_string()));
        assert_eq!(map.get(&true_1), Some(&"true again".to_string()));
        assert_eq!(map.get(&false_1), Some(&"false".to_string()));
        assert_eq!(map.get(&true_2), Some(&"true again".to_string()));
    }

    #[test]
    fn tests_is_hashable() {
        let one = Object::INTEGER(1);
        let two = Object::INTEGER(2);
        let string_1 = Object::STRING("one".to_string());
        let string_2 = Object::STRING("two".to_string());
        let true_1 = Object::BOOLEAN(true);
        let false_1 = Object::BOOLEAN(false);
        let return_object = Object::RETURN(Box::new(Object::INTEGER(1)));

        assert!(one.is_hashable());
        assert!(two.is_hashable());
        assert!(string_1.is_hashable());
        assert!(string_2.is_hashable());
        assert!(true_1.is_hashable());
        assert!(false_1.is_hashable());
        assert!(!return_object.is_hashable());
    }
}
