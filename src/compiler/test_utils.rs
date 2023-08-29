use std::rc::Rc;

use crate::{
    compiler::{code::Instructions, Compiler},
    object::{test_utils::check_constants, Object},
    parser::parse,
};

#[allow(dead_code)]
pub(crate) fn check_instructions(instructions: &Instructions, expected: &Instructions) {
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

#[allow(dead_code)]
pub(crate) struct CompilerTestCase {
    pub(crate) input: String,
    pub(crate) expected_constants: Vec<Object>,
    pub(crate) expected_instructions: Instructions,
}

#[allow(dead_code)]
pub(crate) fn flatten_instructions(instructions: Vec<Instructions>) -> Instructions {
    let mut res = Instructions::default();
    for instruction in instructions {
        res.append(instruction);
    }
    res
}

#[allow(dead_code)]
pub(crate) fn flatten_u8_instructions(instructions: Vec<Instructions>) -> Vec<u8> {
    let mut res = vec![];
    for instruction in instructions {
        res.append(&mut instruction.data.clone());
    }
    res
}

#[allow(dead_code)]
pub(crate) fn run_compiler(tests: Vec<CompilerTestCase>) {
    for test in tests {
        println!("Testing input: {}", test.input);
        let program = parse(&test.input);

        let mut compiler = Compiler::new();

        match compiler.compile(program) {
            Ok(_) => {
                let bytecode = compiler.bytecode();
                println!(
                    "want {}, got {}",
                    test.expected_instructions, bytecode.instructions
                );
                check_instructions(&bytecode.instructions, &test.expected_instructions);
                check_constants(
                    &bytecode.constants,
                    &test
                        .expected_constants
                        .iter()
                        .map(|x| Rc::new(x.clone()))
                        .collect(),
                );
            }
            Err(err) => panic!("compiler error: {err}"),
        }
    }
}
