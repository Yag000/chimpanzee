#[allow(clippy::too_many_lines)]
#[cfg(test)]
pub mod tests {

    use std::rc::Rc;

    use object::object::{CompiledFunction, Object};
    use object::test_utils::check_constants;
    use parser::parser::parse;

    use crate::code::{Instructions, Opcode};
    use crate::compiler::Compiler;
    use crate::test_utils::check_instructions;

    struct CompilerTestCase {
        input: String,
        expected_constants: Vec<Object>,
        expected_instructions: Instructions,
    }

    fn flatten_instructions(instructions: Vec<Instructions>) -> Instructions {
        let mut res = Instructions::default();
        for instruction in instructions {
            res.append(instruction);
        }
        res
    }

    fn flatten_u8_instructions(instructions: Vec<Instructions>) -> Vec<u8> {
        let mut res = vec![];
        for instruction in instructions {
            res.append(&mut instruction.data.clone());
        }
        res
    }

    fn run_compiler(tests: Vec<CompilerTestCase>) {
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
    fn test_funtions() {
        let tests = vec![
            CompilerTestCase {
                input: "fn() { return 5 + 10; }".to_string(),
                expected_constants: vec![
                    Object::INTEGER(5),
                    Object::INTEGER(10),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::Constant.make(vec![0]),
                            Opcode::Constant.make(vec![1]),
                            Opcode::Add.make(vec![]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 0,
                        num_parameters: 0,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![2, 0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "fn() { 5 + 10; }".to_string(),
                expected_constants: vec![
                    Object::INTEGER(5),
                    Object::INTEGER(10),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::Constant.make(vec![0]),
                            Opcode::Constant.make(vec![1]),
                            Opcode::Add.make(vec![]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 0,
                        num_parameters: 0,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![2, 0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "fn() { 1; 2 }".to_string(),
                expected_constants: vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::Constant.make(vec![0]),
                            Opcode::Pop.make(vec![]),
                            Opcode::Constant.make(vec![1]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 0,
                        num_parameters: 0,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![2, 0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "fn() { 24 }()".to_string(),
                expected_constants: vec![
                    Object::INTEGER(24),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::Constant.make(vec![0]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 0,
                        num_parameters: 0,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![1, 0]),
                    Opcode::Call.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "let noArg = fn() { 24 }; noArg();".to_string(),
                expected_constants: vec![
                    Object::INTEGER(24),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::Constant.make(vec![0]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 0,
                        num_parameters: 0,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![1, 0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::GetGlobal.make(vec![0]),
                    Opcode::Call.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "let oneArg = fn(a) {}; oneArg(24);".to_string(),
                expected_constants: vec![
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![Opcode::Return.make(vec![0])]),
                        num_locals: 1,
                        num_parameters: 1,
                    }),
                    Object::INTEGER(24),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![0, 0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::GetGlobal.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Call.make(vec![1]),
                    Opcode::Pop.make(vec![0]),
                ]),
            },
            CompilerTestCase {
                input: "let manyArg = fn(a, b, c) { }; manyArg(24, 25, 26);".to_string(),
                expected_constants: vec![
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![Opcode::Return.make(vec![0])]),
                        num_locals: 3,
                        num_parameters: 3,
                    }),
                    Object::INTEGER(24),
                    Object::INTEGER(25),
                    Object::INTEGER(26),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![0, 0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::GetGlobal.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Constant.make(vec![2]),
                    Opcode::Constant.make(vec![3]),
                    Opcode::Call.make(vec![3]),
                    Opcode::Pop.make(vec![0]),
                ]),
            },
            CompilerTestCase {
                input: "let oneArg = fn(a) { a; }; oneArg(24);".to_string(),
                expected_constants: vec![
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::ReturnValue.make(vec![0]),
                        ]),
                        num_locals: 1,
                        num_parameters: 1,
                    }),
                    Object::INTEGER(24),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![0, 0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::GetGlobal.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Call.make(vec![1]),
                    Opcode::Pop.make(vec![0]),
                ]),
            },
            CompilerTestCase {
                input: "let manyArg = fn(a, b, c) { a; b; c; }; manyArg(24, 25, 26);".to_string(),
                expected_constants: vec![
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Pop.make(vec![0]),
                            Opcode::GetLocal.make(vec![1]),
                            Opcode::Pop.make(vec![0]),
                            Opcode::GetLocal.make(vec![2]),
                            Opcode::ReturnValue.make(vec![0]),
                        ]),
                        num_locals: 3,
                        num_parameters: 3,
                    }),
                    Object::INTEGER(24),
                    Object::INTEGER(25),
                    Object::INTEGER(26),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![0, 0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::GetGlobal.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Constant.make(vec![2]),
                    Opcode::Constant.make(vec![3]),
                    Opcode::Call.make(vec![3]),
                    Opcode::Pop.make(vec![0]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_function_with_no_return_value() {
        let tests = vec![CompilerTestCase {
            input: "fn() { }".to_string(),
            expected_constants: vec![Object::COMPILEDFUNCTION(CompiledFunction {
                instructions: flatten_u8_instructions(vec![Opcode::Return.make(vec![])]),
                num_locals: 0,
                num_parameters: 0,
            })],
            expected_instructions: flatten_instructions(vec![
                Opcode::Closure.make(vec![0, 0]),
                Opcode::Pop.make(vec![]),
            ]),
        }];

        run_compiler(tests);
    }

    #[test]
    fn test_let_statements_scope() {
        let tests = vec![
            CompilerTestCase {
                input: r#"
                let num = 55;
                fn() { num }"#
                    .to_string(),
                expected_constants: vec![
                    Object::INTEGER(55),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::GetGlobal.make(vec![0]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 0,
                        num_parameters: 0,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::Closure.make(vec![1, 0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: r#"
                fn() { 
                    let num = 55;
                    num
                }"#
                .to_string(),
                expected_constants: vec![
                    Object::INTEGER(55),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::Constant.make(vec![0]),
                            Opcode::SetLocal.make(vec![0]),
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 0,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![1, 0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: r#"
                fn() { 
                    let a = 55;
                    let b = 77;
                    a + b
                }"#
                .to_string(),
                expected_constants: vec![
                    Object::INTEGER(55),
                    Object::INTEGER(77),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::Constant.make(vec![0]),
                            Opcode::SetLocal.make(vec![0]),
                            Opcode::Constant.make(vec![1]),
                            Opcode::SetLocal.make(vec![1]),
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::GetLocal.make(vec![1]),
                            Opcode::Add.make(vec![]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 2,
                        num_parameters: 0,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![2, 0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_builtins() {
        let tests = vec![
            CompilerTestCase {
                input: "len([]); push([], 1);".to_string(),
                expected_constants: vec![Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::GetBuiltin.make(vec![0]),
                    Opcode::Array.make(vec![0]),
                    Opcode::Call.make(vec![1]),
                    Opcode::Pop.make(vec![]),
                    Opcode::GetBuiltin.make(vec![4]),
                    Opcode::Array.make(vec![0]),
                    Opcode::Constant.make(vec![0]),
                    Opcode::Call.make(vec![2]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "fn() { len([]); }".to_string(),
                expected_constants: vec![Object::COMPILEDFUNCTION(CompiledFunction {
                    instructions: flatten_u8_instructions(vec![
                        Opcode::GetBuiltin.make(vec![0]),
                        Opcode::Array.make(vec![0]),
                        Opcode::Call.make(vec![1]),
                        Opcode::ReturnValue.make(vec![]),
                    ]),
                    num_locals: 0,
                    num_parameters: 0,
                })],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![0, 0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]

    fn test_closures() {
        let tests = vec![
            CompilerTestCase {
                input: r#"
                fn(a){
                    fn(b){
                        a + b
                    }
                }"#
                .to_string(),
                expected_constants: vec![
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::GetFree.make(vec![0]),
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Add.make(vec![]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 1,
                    }),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Closure.make(vec![0, 1]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 1,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![1, 0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: r#"
                    fn(a) {
                        fn(b) {
                            fn(c) {
                                a + b + c
                            }
                        }
                    };"#
                .to_string(),

                expected_constants: vec![
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::GetFree.make(vec![0]),
                            Opcode::GetFree.make(vec![1]),
                            Opcode::Add.make(vec![]),
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Add.make(vec![]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 1,
                    }),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::GetFree.make(vec![0]),
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Closure.make(vec![0, 2]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 1,
                    }),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Closure.make(vec![1, 1]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 1,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![2, 0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: r#"
                    let global = 55;
                   fn() {
                        let a = 66;
                        fn() {
                            let b = 77;
                            fn() {
                                let c = 88;
                                global + a + b + c;
                            }
                        }
                    }
                    "#
                .to_string(),
                expected_constants: vec![
                    Object::INTEGER(55),
                    Object::INTEGER(66),
                    Object::INTEGER(77),
                    Object::INTEGER(88),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::Constant.make(vec![3]),
                            Opcode::SetLocal.make(vec![0]),
                            Opcode::GetGlobal.make(vec![0]),
                            Opcode::GetFree.make(vec![0]),
                            Opcode::Add.make(vec![]),
                            Opcode::GetFree.make(vec![1]),
                            Opcode::Add.make(vec![]),
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Add.make(vec![]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 0,
                    }),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::Constant.make(vec![2]),
                            Opcode::SetLocal.make(vec![0]),
                            Opcode::GetFree.make(vec![0]),
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Closure.make(vec![4, 2]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 0,
                    }),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::Constant.make(vec![1]),
                            Opcode::SetLocal.make(vec![0]),
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Closure.make(vec![5, 1]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 0,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::Closure.make(vec![6, 0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_recursive_functions() {
        let tests = vec![
            CompilerTestCase {
                input: r#"
                let countDown = fn(x){
                    countDown(x - 1);
                };
                countDown(1);"#
                    .to_string(),
                expected_constants: vec![
                    Object::INTEGER(1),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::CurrentClosure.make(vec![]),
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Constant.make(vec![0]),
                            Opcode::Sub.make(vec![]),
                            Opcode::Call.make(vec![1]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 1,
                    }),
                    Object::INTEGER(1),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![1, 0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::GetGlobal.make(vec![0]),
                    Opcode::Constant.make(vec![2]),
                    Opcode::Call.make(vec![1]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: r#"
                let wrapper = fn() {
                    let countDown = fn(x) {
                        countDown(x - 1);
                    };
                    countDown(1);
                };
                wrapper();
                "#
                .to_string(),
                expected_constants: vec![
                    Object::INTEGER(1),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::CurrentClosure.make(vec![]),
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Constant.make(vec![0]),
                            Opcode::Sub.make(vec![]),
                            Opcode::Call.make(vec![1]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 1,
                    }),
                    Object::INTEGER(1),
                    Object::COMPILEDFUNCTION(CompiledFunction {
                        instructions: flatten_u8_instructions(vec![
                            Opcode::Closure.make(vec![1, 0]),
                            Opcode::SetLocal.make(vec![0]),
                            Opcode::GetLocal.make(vec![0]),
                            Opcode::Constant.make(vec![2]),
                            Opcode::Call.make(vec![1]),
                            Opcode::ReturnValue.make(vec![]),
                        ]),
                        num_locals: 1,
                        num_parameters: 0,
                    }),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Closure.make(vec![3, 0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::GetGlobal.make(vec![0]),
                    Opcode::Call.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }
}
