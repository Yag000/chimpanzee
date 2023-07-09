use std::rc::Rc;

use interpreter::object::Object;
use lexer::lexer::Lexer;
use parser::{ast::Program, parser::Parser};

use crate::code::Instructions;

pub fn parse(input: &str) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse_program()
}

pub fn check_instructions(instructions: &Instructions, expected: &Instructions) {
    assert_eq!(
        instructions.data.len(),
        expected.data.len(),
        "wrong instructions length"
    );
    assert_eq!(
        instructions, expected,
        "wrong instructions. want={:?}, got={:?}",
        expected, instructions
    );
}

pub fn check_constants(constants: &Vec<Object>, expected: &Vec<Rc<Object>>) {
    assert_eq!(
        constants.len(),
        expected.len(),
        "wrong number of constants. got={:?}, want={:?}",
        constants.len(),
        expected.len()
    );

    for (i, constant) in constants.iter().enumerate() {
        match constant {
            Object::INTEGER(x) => check_integer_object(x, &expected[i]),
            Object::BOOLEAN(x) => check_boolean_object(x, &expected[i]),
            Object::NULL => assert_eq!(
                constant,
                &Object::NULL,
                "constant[{}] - wrong type. got={:?}",
                i,
                constant
            ),
            Object::STRING(x) => check_string_object(x, &expected[i]),
            _ => panic!("constant[{}] - wrong type. got={:?}", i, constant),
        }
    }
}

pub fn check_integer_object(integer: &i64, expected: &Object) {
    match expected {
        Object::INTEGER(i) => assert_eq!(
            integer, i,
            "integer object has wrong value. got={}, want={}",
            integer, i
        ),
        _ => panic!("object is not Integer. got={:?}", expected),
    }
}

pub fn check_boolean_object(boolean: &bool, expected: &Object) {
    match expected {
        Object::BOOLEAN(b) => assert_eq!(
            boolean, b,
            "boolean object has wrong value. got={}, want={}",
            boolean, b
        ),
        _ => panic!("object is not Boolean. got={:?}", expected),
    }
}

pub fn check_string_object(string: &str, expected: &Object) {
    match expected {
        Object::STRING(s) => assert_eq!(
            string, s,
            "string object has wrong value. got={}, want={}",
            string, s
        ),
        _ => panic!("object is not String. got={:?}", expected),
    }
}
