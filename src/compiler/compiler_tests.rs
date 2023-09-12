#[allow(clippy::too_many_lines)]
#[cfg(test)]
pub mod tests {

    use crate::{
        compiler::{
            code::Opcode,
            test_utils::{flatten_instructions, run_compiler, CompilerTestCase},
        },
        object::Object,
    };

    #[test]
    fn test_integer_arithemtic() {
        let tests = vec![
            CompilerTestCase {
                input: "1 + 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Add.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1; 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 * 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Mul.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 / 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Div.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 - 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Sub.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "-1".to_string(),
                expected_constants: vec![Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Minus.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 % 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Modulo.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_boolean_expression() {
        let tests = vec![
            CompilerTestCase {
                input: "true".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::True.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "false".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::False.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_boolean_logic() {
        let tests = vec![
            CompilerTestCase {
                input: "1 > 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::GreaterThan.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 >= 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::GreaterEqualThan.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 < 2".to_string(),
                expected_constants: vec![Object::INTEGER(2), Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::GreaterThan.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 <= 2".to_string(),
                expected_constants: vec![Object::INTEGER(2), Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::GreaterEqualThan.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 == 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Equal.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 != 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::NotEqual.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "true == false".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::True.make(vec![]),
                    Opcode::False.make(vec![]),
                    Opcode::Equal.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "true != false".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::True.make(vec![]),
                    Opcode::False.make(vec![]),
                    Opcode::NotEqual.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "!true".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::True.make(vec![]),
                    Opcode::Bang.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "!false".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::False.make(vec![]),
                    Opcode::Bang.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_conditionals() {
        let tests = vec![
            CompilerTestCase {
                input: "if (true) { 10 }; 3333;".to_string(),
                expected_constants: vec![Object::INTEGER(10), Object::INTEGER(3333)],
                expected_instructions: flatten_instructions(vec![
                    // 0000
                    Opcode::True.make(vec![]),
                    // 0001
                    Opcode::JumpNotTruthy.make(vec![10]),
                    // 0004
                    Opcode::Constant.make(vec![0]),
                    // 0007
                    Opcode::Jump.make(vec![11]),
                    // 0010
                    Opcode::Null.make(vec![]),
                    // 0011
                    Opcode::Pop.make(vec![]),
                    // 0012
                    Opcode::Constant.make(vec![1]),
                    // 0015
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "if (true) { 10 } else { 20 }; 3333;".to_string(),
                expected_constants: vec![
                    Object::INTEGER(10),
                    Object::INTEGER(20),
                    Object::INTEGER(3333),
                ],
                expected_instructions: flatten_instructions(vec![
                    // 0000
                    Opcode::True.make(vec![]),
                    // 0001
                    Opcode::JumpNotTruthy.make(vec![10]),
                    // 0004
                    Opcode::Constant.make(vec![0]),
                    // 0007
                    Opcode::Jump.make(vec![13]),
                    // 0010
                    Opcode::Constant.make(vec![1]),
                    // 0013
                    Opcode::Pop.make(vec![]),
                    // 0014
                    Opcode::Constant.make(vec![2]),
                    // 0017
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }
    #[test]
    fn test_let_statements() {
        let tests = vec![
            CompilerTestCase {
                input: r#"
                let one = 1;"#
                    .to_string(),
                expected_constants: vec![Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::SetGlobal.make(vec![0]),
                ]),
            },
            CompilerTestCase {
                input: r#"
                let one = 1;    
                let two = 2"#
                    .to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::SetGlobal.make(vec![1]),
                ]),
            },
            CompilerTestCase {
                input: r#"
                let one = 1;
                one;"#
                    .to_string(),
                expected_constants: vec![Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::GetGlobal.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: r#"
                let one = 1;
                let two = one;
                two;"#
                    .to_string(),
                expected_constants: vec![Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::GetGlobal.make(vec![0]),
                    Opcode::SetGlobal.make(vec![1]),
                    Opcode::GetGlobal.make(vec![1]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_string_expressions() {
        let tests = vec![
            CompilerTestCase {
                input: r#""monkey""#.to_string(),
                expected_constants: vec![Object::STRING("monkey".to_string())],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: r#""mon" + "key""#.to_string(),
                expected_constants: vec![
                    Object::STRING("mon".to_string()),
                    Object::STRING("key".to_string()),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Add.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_array_expressions() {
        let tests = vec![
            CompilerTestCase {
                input: "[]".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Array.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "[1, 2, 3]".to_string(),
                expected_constants: vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::INTEGER(3),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Constant.make(vec![2]),
                    Opcode::Array.make(vec![3]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "[1 + 2, 3 - 4, 5 * 6]".to_string(),
                expected_constants: vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::INTEGER(3),
                    Object::INTEGER(4),
                    Object::INTEGER(5),
                    Object::INTEGER(6),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Add.make(vec![]),
                    Opcode::Constant.make(vec![2]),
                    Opcode::Constant.make(vec![3]),
                    Opcode::Sub.make(vec![]),
                    Opcode::Constant.make(vec![4]),
                    Opcode::Constant.make(vec![5]),
                    Opcode::Mul.make(vec![]),
                    Opcode::Array.make(vec![3]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_hash_expression() {
        let tests = vec![
            CompilerTestCase {
                input: "{}".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::HashMap.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "{1: 2, 3: 4, 5: 6}".to_string(),
                expected_constants: vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::INTEGER(3),
                    Object::INTEGER(4),
                    Object::INTEGER(5),
                    Object::INTEGER(6),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Constant.make(vec![2]),
                    Opcode::Constant.make(vec![3]),
                    Opcode::Constant.make(vec![4]),
                    Opcode::Constant.make(vec![5]),
                    Opcode::HashMap.make(vec![6]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "{1: 2 + 3, 4: 5 * 6}".to_string(),
                expected_constants: vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::INTEGER(3),
                    Object::INTEGER(4),
                    Object::INTEGER(5),
                    Object::INTEGER(6),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Constant.make(vec![2]),
                    Opcode::Add.make(vec![]),
                    Opcode::Constant.make(vec![3]),
                    Opcode::Constant.make(vec![4]),
                    Opcode::Constant.make(vec![5]),
                    Opcode::Mul.make(vec![]),
                    Opcode::HashMap.make(vec![4]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_index_expressions() {
        let tests = vec![
            CompilerTestCase {
                input: "[1, 2, 3][1 + 1]".to_string(),
                expected_constants: vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::INTEGER(3),
                    Object::INTEGER(1),
                    Object::INTEGER(1),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Constant.make(vec![2]),
                    Opcode::Array.make(vec![3]),
                    Opcode::Constant.make(vec![3]),
                    Opcode::Constant.make(vec![4]),
                    Opcode::Add.make(vec![]),
                    Opcode::Index.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "{1: 2}[2 - 1]".to_string(),
                expected_constants: vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::INTEGER(2),
                    Object::INTEGER(1),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::HashMap.make(vec![2]),
                    Opcode::Constant.make(vec![2]),
                    Opcode::Constant.make(vec![3]),
                    Opcode::Sub.make(vec![]),
                    Opcode::Index.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_shadowing_with_itself() {
        let tests = vec![CompilerTestCase {
            input: r#"
                let a = 1;
                let a = a + 1;"#
                .to_string(),

            expected_constants: vec![Object::INTEGER(1), Object::INTEGER(1)],
            expected_instructions: flatten_instructions(vec![
                Opcode::Constant.make(vec![0]),
                Opcode::SetGlobal.make(vec![0]),
                Opcode::GetGlobal.make(vec![0]),
                Opcode::Constant.make(vec![1]),
                Opcode::Add.make(vec![]),
                Opcode::SetGlobal.make(vec![0]),
            ]),
        }];

        run_compiler(tests);
    }
}
