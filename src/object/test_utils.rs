use std::rc::Rc;

use crate::object::Object;

#[allow(clippy::useless_vec, clippy::ptr_arg)] // TODO: Make this cleaner
pub fn check_constants(constants: &[Object], expected: &Vec<Rc<Object>>) {
    assert_eq!(
        constants.len(),
        expected.len(),
        "wrong number of constants. got={}, want={}",
        constants.len(),
        expected.len()
    );

    for (expected_constant, constant) in expected.iter().zip(constants.iter()) {
        assert_eq!(
            **expected_constant, *constant,
            "constant not equal. got={constant}, want={expected_constant}"
        );
    }
}
