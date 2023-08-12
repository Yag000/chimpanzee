use crate::compiler::code::Instructions;

#[allow(dead_code)]
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
