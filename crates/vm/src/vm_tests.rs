#[allow(clippy::too_many_lines)]
#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use compiler::compiler::Compiler;
    use object::{object::Object, test_utils::check_constants};
    use parser::parser::parse;

    use crate::vm::VM;

    struct VmTestCase {
        input: String,
        expected: Object,
    }

    fn run_vm_tests(tests: Vec<VmTestCase>) {
        for test in tests {
            println!("Running test: {}", test.input);
            let program = parse(&test.input);
            let mut compiler = Compiler::new();
            compiler.compile(program).unwrap();
            let bytecode = compiler.bytecode();

            let mut vm = VM::new(bytecode);
            vm.run().unwrap();
            let got = vm.last_popped_stack_element().unwrap();
            check_constants(&vec![test.expected], &vec![got]);
        }
    }

    #[test]
    fn test_integer_arithmetic() {
        let tests = vec![
            VmTestCase {
                input: "1".to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: "2".to_string(),
                expected: Object::INTEGER(2),
            },
            VmTestCase {
                input: "1 + 2".to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: "1 - 2".to_string(),
                expected: Object::INTEGER(-1),
            },
            VmTestCase {
                input: "1 * 2".to_string(),
                expected: Object::INTEGER(2),
            },
            VmTestCase {
                input: "4 / 2".to_string(),
                expected: Object::INTEGER(2),
            },
            VmTestCase {
                input: "50 / 2 * 2 + 10 - 5".to_string(),
                expected: Object::INTEGER(55),
            },
            VmTestCase {
                input: "5 + 5 + 5 + 5 - 10".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "2 * 2 * 2 * 2 * 2".to_string(),
                expected: Object::INTEGER(32),
            },
            VmTestCase {
                input: "5 * 2 + 10".to_string(),
                expected: Object::INTEGER(20),
            },
            VmTestCase {
                input: "-1".to_string(),
                expected: Object::INTEGER(-1),
            },
            VmTestCase {
                input: "-10".to_string(),
                expected: Object::INTEGER(-10),
            },
            VmTestCase {
                input: "-50 + 100 + -50".to_string(),
                expected: Object::INTEGER(0),
            },
            VmTestCase {
                input: "(5 + 10 * 2 + 15 / 3) * 2 + -10".to_string(),
                expected: Object::INTEGER(50),
            },
        ];
        run_vm_tests(tests);
    }

    #[test]
    fn test_boolean_logic() {
        let tests = vec![
            VmTestCase {
                input: "true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "1 < 2".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "1 <= 2".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "2 <= 2".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "1 > 2".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "1 >= 2".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "2 >= 2".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "1 == 2".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "1 != 2".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "true == true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "false == false".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "true == false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "true && true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "true && false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "false && true".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "false && false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "true || true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "true || false".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "false || true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "false || false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "!true".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "!false".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "!5".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "!!true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "!!false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "!!5".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "!(if (false) { 5 })".to_string(),
                expected: Object::BOOLEAN(true),
            },
        ];
        run_vm_tests(tests);
    }
    #[test]
    fn test_conditionals() {
        let tests = vec![
            VmTestCase {
                input: "if (true) { 10 }".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "if (true) { 10 } else { 20 }".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "if (false) { 10 } else { 20 } ".to_string(),
                expected: Object::INTEGER(20),
            },
            VmTestCase {
                input: "if (1) { 10 }".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "if (1 < 2) { 10 }".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "if (1 < 2) { 10 } else { 20 }".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "if (1 > 2) { 10 } else { 20 }".to_string(),
                expected: Object::INTEGER(20),
            },
            VmTestCase {
                input: "if (1 > 2) { 10 }".to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: "if (false) { 10 }".to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: "if ((if (false) { 10 })) { 10 } else { 20 }".to_string(),
                expected: Object::INTEGER(20),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_global_let_statements() {
        let tests = vec![
            VmTestCase {
                input: "let one = 1; one".to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: "let one = 1; let two = 2; one + two".to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: "let one = 1; let two = one + one; one + two".to_string(),
                expected: Object::INTEGER(3),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_string_expressions() {
        let tests = vec![
            VmTestCase {
                input: "\"monkey\"".to_string(),
                expected: Object::STRING("monkey".to_string()),
            },
            VmTestCase {
                input: "\"mon\" + \"key\"".to_string(),
                expected: Object::STRING("monkey".to_string()),
            },
            VmTestCase {
                input: "\"mon\" + \"key\" + \"banana\"".to_string(),
                expected: Object::STRING("monkeybanana".to_string()),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_array_expressions() {
        let tests = vec![
            VmTestCase {
                input: "[]".to_string(),
                expected: Object::ARRAY(vec![]),
            },
            VmTestCase {
                input: "[1, 2, 3]".to_string(),
                expected: Object::ARRAY(vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::INTEGER(3),
                ]),
            },
            VmTestCase {
                input: "[1 + 2, 3 * 4, 5 + 6]".to_string(),
                expected: Object::ARRAY(vec![
                    Object::INTEGER(3),
                    Object::INTEGER(12),
                    Object::INTEGER(11),
                ]),
            },
            VmTestCase {
                input: "[\"yes\", false, [1,2]]".to_string(),
                expected: Object::ARRAY(vec![
                    Object::STRING("yes".to_string()),
                    Object::BOOLEAN(false),
                    Object::ARRAY(vec![Object::INTEGER(1), Object::INTEGER(2)]),
                ]),
            },
        ];

        run_vm_tests(tests);
    }
    #[test]
    fn test_hashmap_expressions() {
        let tests = vec![
            VmTestCase {
                input: "{}".to_string(),
                expected: Object::HASHMAP(HashMap::new()),
            },
            VmTestCase {
                input: "{1:2, 2:3}".to_string(),
                expected: Object::HASHMAP(
                    vec![
                        (Object::INTEGER(1), Object::INTEGER(2)),
                        (Object::INTEGER(2), Object::INTEGER(3)),
                    ]
                    .into_iter()
                    .collect(),
                ),
            },
            VmTestCase {
                input: "{1+1:2, 2*2:3}".to_string(),
                expected: Object::HASHMAP(
                    vec![
                        (Object::INTEGER(2), Object::INTEGER(2)),
                        (Object::INTEGER(4), Object::INTEGER(3)),
                    ]
                    .into_iter()
                    .collect(),
                ),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_index_expression() {
        let tests = vec![
            VmTestCase {
                input: "[1, 2, 3][1]".to_string(),
                expected: Object::INTEGER(2),
            },
            VmTestCase {
                input: "[1, 2, 3][0 + 2]".to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: "[[1, 1, 1]][0][0]".to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: "[][0]".to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: "[1, 2, 3][99]".to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: "[1][-1]".to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: "{1: 1, 2: 2}[1]".to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: "{1: 1, 2: 2}[2]".to_string(),
                expected: Object::INTEGER(2),
            },
            VmTestCase {
                input: "{1: 1}[0]".to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: "{}[0]".to_string(),
                expected: Object::NULL,
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_calling_functions_without_arguments() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let fivePlusTen = fn() { 5 + 10; };
                    fivePlusTen();"#
                    .to_string(),
                expected: Object::INTEGER(15),
            },
            VmTestCase {
                input: r#"
                    let one = fn() { 1; };
                    let two = fn() { 2; };
                    one() + two()"#
                    .to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r#"
                    let a = fn() { 1 };
                    let b = fn() { a() + 1 };
                    let c = fn() { b() + 1 };
                    c();"#
                    .to_string(),
                expected: Object::INTEGER(3),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_functions_with_return_statements() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let earlyExit = fn() { return 99; 100; };
                    earlyExit();"#
                    .to_string(),
                expected: Object::INTEGER(99),
            },
            VmTestCase {
                input: r#"
                    let earlyExit = fn() { return 99; return 100; };
                    earlyExit();"#
                    .to_string(),
                expected: Object::INTEGER(99),
            },
        ];
        run_vm_tests(tests);
    }

    #[test]
    fn test_functions_without_return_value() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let noReturn = fn() { };
                    noReturn();"#
                    .to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: r#"
                    let noReturn = fn() { };
                    let noReturnTwo = fn() { noReturn(); };
                    noReturn();
                    noReturnTwo();"#
                    .to_string(),
                expected: Object::NULL,
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_first_class_functions() {
        let tests = vec![
            VmTestCase {
                input: r#"
                let returnsOne = fn() { 1; };
                let returnsOneReturner = fn() { returnsOne; };
                returnsOneReturner()();"#
                    .to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: r#"
                    let returnsOneReturner = fn() {
                        let returnsOne = fn() { 1; };
                        returnsOne;
                    };
                    returnsOneReturner()();"#
                    .to_string(),
                expected: Object::INTEGER(1),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_calling_function_with_bindings() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let one = fn() { let one = 1; one };
                    one();"#
                    .to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: r#"
                    let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
                    oneAndTwo();"#
                    .to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r#"
                    let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
                    let threeAndFour = fn() { let three = 3; let four = 4; three + four; };
                    oneAndTwo() + threeAndFour();"#
                    .to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: r#"
                    let firstFoobar = fn() { let foobar = 50; foobar; };
                    let secondFoobar = fn() { let foobar = 100; foobar; };
                    firstFoobar() + secondFoobar();"#
                    .to_string(),
                expected: Object::INTEGER(150),
            },
            VmTestCase {
                input: r#"
                    let globalSeed = 50;
                    let minusOne = fn() {
                        let num = 1;
                        globalSeed - num;
                    }
                    let minusTwo = fn() {
                        let num = 2;
                        globalSeed - num;
                    }
                    minusOne() + minusTwo();"#
                    .to_string(),
                expected: Object::INTEGER(97),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_calling_functions_with_arguments_and_bindings() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let identity = fn(a) { a; };
                    identity(4);"#
                    .to_string(),
                expected: Object::INTEGER(4),
            },
            VmTestCase {
                input: r#"
                    let sum = fn(a, b) { a + b; };
                    sum(1, 2);"#
                    .to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r#"
                    let sum = fn(a, b) {
                        let c = a + b;
                        c;
                    };
                    sum(1, 2);"#
                    .to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r#"
                    let sum = fn(a, b) {
                        let c = a + b;
                        c;
                    };
                    sum(1, 2) + sum(3, 4);"#
                    .to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: r#"
                    let sum = fn(a, b) {
                        let c = a + b;
                        c;
                    };
                    let outer = fn() {
                        sum(1, 2) + sum(3, 4);
                    };
                    outer();"#
                    .to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: r#"
                    let globalNum = 10;
                    let sum = fn(a, b) {
                        let c = a + b;
                        c + globalNum;
                    };
                    let outer = fn() {
                        sum(1, 2) + sum(3, 4) + globalNum;
                    };
                    outer() + globalNum;"#
                    .to_string(),
                expected: Object::INTEGER(50),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_calling_functions_with_wrong_arguments() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    fn() { 1; }(1);"#
                    .to_string(),
                expected: Object::ERROR("Wrong number of arguments: want=0, got=1".to_string()),
            },
            VmTestCase {
                input: r#"
                    fn(a) { a; }();"#
                    .to_string(),
                expected: Object::ERROR("Wrong number of arguments: want=1, got=0".to_string()),
            },
            VmTestCase {
                input: r#"
                    fn(a, b) { a + b; }(1);"#
                    .to_string(),
                expected: Object::ERROR("Wrong number of arguments: want=2, got=1".to_string()),
            },
        ];

        for test in tests {
            println!("Running test: {}", test.input);
            let program = parse(&test.input);
            let mut compiler = Compiler::new();
            compiler.compile(program).unwrap();
            let bytecode = compiler.bytecode();

            let mut vm = VM::new(bytecode);
            match vm.run() {
                Ok(_) => {
                    panic!("Expected error, but got no error");
                }
                Err(e) => match test.expected {
                    Object::ERROR(msg) => {
                        assert_eq!(e, msg);
                    }
                    _ => {
                        unreachable!("Poorly written test, the expected value should be an error");
                    }
                },
            }
        }
    }
}
