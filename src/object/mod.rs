pub mod builtins;
pub mod enviroment;
pub mod test_utils;

use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    hash::Hash,
    rc::Rc,
};

use crate::parser::ast::{BlockStatement, Identifier};

use crate::object::{builtins::BuiltinFunction, enviroment::Environment};

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
    CLOSURE(Closure),
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
            Object::CLOSURE(o) => write!(f, "{o}"),
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
            Object::CLOSURE(_) => String::from("CLOSURE"),
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

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledFunction {
    pub instructions: Vec<u8>,
    pub num_locals: usize,
    pub num_parameters: usize,
}

impl Display for CompiledFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CompiledFunction(num_locals={}, num_parameters={}, instructions={:?})",
            self.num_locals, self.num_parameters, self.instructions
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub function: CompiledFunction,
    pub free: Vec<Object>,
}

impl Display for Closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Closure(function={}, free={:?}",
            self.function, self.free
        )
    }
}

impl Closure {
    pub fn new(function: CompiledFunction) -> Self {
        Self {
            function,
            free: Vec::new(),
        }
    }

    pub fn add_free_variable(&mut self, variable: Object) {
        self.free.push(variable);
    }

    pub fn extend_free_varaibles(&mut self, variables: Vec<Object>) {
        self.free.extend(variables);
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
