use crate::code::Instructions;
use interpreter::object::Object;
use lexer::lexer::Lexer;
use parser::{ast::Program, parser::Parser};
use std::rc::Rc;

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
        "wrong instructions. want={expected:?}, got={instructions:?}"
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

    for (expected_constant, constant) in expected.iter().zip(constants.iter()) {
        assert_eq!(
            **expected_constant, *constant,
            "constant not equal. got={constant:?}, want={expected_constant:?}"
        );
    }
}
