#[allow(clippy::too_many_lines)]
#[cfg(test)]
pub mod tests {

    use crate::{
        compiler::{
            code::Opcode,
            test_utils::{
                flatten_instructions, flatten_u8_instructions, run_compiler, CompilerTestCase,
            },
        },
        object::{CompiledFunction, Object},
    };

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
                input: r"
                let num = 55;
                fn() { num }"
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
                input: r"
                fn() { 
                    let num = 55;
                    num
                }"
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
                input: r"
                fn() { 
                    let a = 55;
                    let b = 77;
                    a + b
                }"
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
                input: r"
                fn(a){
                    fn(b){
                        a + b
                    }
                }"
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
                input: r"
                    fn(a) {
                        fn(b) {
                            fn(c) {
                                a + b + c
                            }
                        }
                    };"
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
                input: r"
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
                    "
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
                input: r"
                let countDown = fn(x){
                    countDown(x - 1);
                };
                countDown(1);"
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
                input: r"
                let wrapper = fn() {
                    let countDown = fn(x) {
                        countDown(x - 1);
                    };
                    countDown(1);
                };
                wrapper();
                "
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
