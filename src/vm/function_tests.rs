#[allow(clippy::too_many_lines)]
#[cfg(test)]
mod tests {
    use crate::{
        compiler::Compiler,
        object::Object,
        parser::parse,
        vm::{
            test_utils::{run_vm_tests, VmTestCase},
            VM,
        },
    };

    #[test]
    fn test_calling_functions_without_arguments() {
        let tests = vec![
            VmTestCase {
                input: r"
                    let fivePlusTen = fn() { 5 + 10; };
                    fivePlusTen();"
                    .to_string(),
                expected: Object::INTEGER(15),
            },
            VmTestCase {
                input: r"
                    let one = fn() { 1; };
                    let two = fn() { 2; };
                    one() + two()"
                    .to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r"
                    let a = fn() { 1 };
                    let b = fn() { a() + 1 };
                    let c = fn() { b() + 1 };
                    c();"
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
                input: r"
                    let earlyExit = fn() { return 99; 100; };
                    earlyExit();"
                    .to_string(),
                expected: Object::INTEGER(99),
            },
            VmTestCase {
                input: r"
                    let earlyExit = fn() { return 99; return 100; };
                    earlyExit();"
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
                input: r"
                    let noReturn = fn() { };
                    noReturn();"
                    .to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: r"
                    let noReturn = fn() { };
                    let noReturnTwo = fn() { noReturn(); };
                    noReturn();
                    noReturnTwo();"
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
                input: r"
                let returnsOne = fn() { 1; };
                let returnsOneReturner = fn() { returnsOne; };
                returnsOneReturner()();"
                    .to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: r"
                    let returnsOneReturner = fn() {
                        let returnsOne = fn() { 1; };
                        returnsOne;
                    };
                    returnsOneReturner()();"
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
                input: r"
                    let one = fn() { let one = 1; one };
                    one();"
                    .to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: r"
                    let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
                    oneAndTwo();"
                    .to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r"
                    let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
                    let threeAndFour = fn() { let three = 3; let four = 4; three + four; };
                    oneAndTwo() + threeAndFour();"
                    .to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: r"
                    let firstFoobar = fn() { let foobar = 50; foobar; };
                    let secondFoobar = fn() { let foobar = 100; foobar; };
                    firstFoobar() + secondFoobar();"
                    .to_string(),
                expected: Object::INTEGER(150),
            },
            VmTestCase {
                input: r"
                    let globalSeed = 50;
                    let minusOne = fn() {
                        let num = 1;
                        globalSeed - num;
                    }
                    let minusTwo = fn() {
                        let num = 2;
                        globalSeed - num;
                    }
                    minusOne() + minusTwo();"
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
                input: r"
                    let identity = fn(a) { a; };
                    identity(4);"
                    .to_string(),
                expected: Object::INTEGER(4),
            },
            VmTestCase {
                input: r"
                    let sum = fn(a, b) { a + b; };
                    sum(1, 2);"
                    .to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r"
                    let sum = fn(a, b) {
                        let c = a + b;
                        c;
                    };
                    sum(1, 2);"
                    .to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r"
                    let sum = fn(a, b) {
                        let c = a + b;
                        c;
                    };
                    sum(1, 2) + sum(3, 4);"
                    .to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: r"
                    let sum = fn(a, b) {
                        let c = a + b;
                        c;
                    };
                    let outer = fn() {
                        sum(1, 2) + sum(3, 4);
                    };
                    outer();"
                    .to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: r"
                    let globalNum = 10;
                    let sum = fn(a, b) {
                        let c = a + b;
                        c + globalNum;
                    };
                    let outer = fn() {
                        sum(1, 2) + sum(3, 4) + globalNum;
                    };
                    outer() + globalNum;"
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
                input: r"
                    fn() { 1; }(1);"
                    .to_string(),
                expected: Object::ERROR("Wrong number of arguments: want=0, got=1".to_string()),
            },
            VmTestCase {
                input: r"
                    fn(a) { a; }();"
                    .to_string(),
                expected: Object::ERROR("Wrong number of arguments: want=1, got=0".to_string()),
            },
            VmTestCase {
                input: r"
                    fn(a, b) { a + b; }(1);"
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
                Ok(()) => {
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

    #[test]
    fn test_builtin_functions() {
        let tests = vec![
            VmTestCase {
                input: r#"len("")"#.to_string(),
                expected: Object::INTEGER(0),
            },
            VmTestCase {
                input: r#"len("four")"#.to_string(),
                expected: Object::INTEGER(4),
            },
            VmTestCase {
                input: r#"len("hello world")"#.to_string(),
                expected: Object::INTEGER(11),
            },
            VmTestCase {
                input: r"len(1)".to_string(),
                expected: Object::ERROR("argument to `len` not supported, got INTEGER".to_string()),
            },
            VmTestCase {
                input: r#"len("one", "two")"#.to_string(),
                expected: Object::ERROR("wrong number of arguments. got=2, want=1".to_string()),
            },
            VmTestCase {
                input: r"len([1, 2, 3])".to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r"len([])".to_string(),
                expected: Object::INTEGER(0),
            },
            VmTestCase {
                input: r"len([1, 2, 3], [4, 5, 6])".to_string(),
                expected: Object::ERROR("wrong number of arguments. got=2, want=1".to_string()),
            },
            VmTestCase {
                input: r"first([1, 2, 3])".to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: r"first([])".to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: r"first(1)".to_string(),
                expected: Object::ERROR(
                    "argument to `first` not supported, must be ARRAY, got INTEGER".to_string(),
                ),
            },
            VmTestCase {
                input: r"last([1, 2, 3])".to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r"last([])".to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: r"last(1)".to_string(),
                expected: Object::ERROR(
                    "argument to `last` not supported, must be ARRAY, got INTEGER".to_string(),
                ),
            },
            VmTestCase {
                input: r"rest([1, 2, 3])".to_string(),
                expected: Object::ARRAY(vec![Object::INTEGER(2), Object::INTEGER(3)]),
            },
            VmTestCase {
                input: r"rest([])".to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: r"push([], 1)".to_string(),
                expected: Object::ARRAY(vec![Object::INTEGER(1)]),
            },
            VmTestCase {
                input: r"push(1, 1)".to_string(),
                expected: Object::ERROR(
                    "argument to `push` not supported, must be ARRAY, got INTEGER".to_string(),
                ),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_closures() {
        let tests = vec![
            VmTestCase {
                input: r"
                let newClosure = fn(a) {
                    fn() { a; };
                };
                let closure = newClosure(99);
                closure();"
                    .to_string(),
                expected: Object::INTEGER(99),
            },
            VmTestCase {
                input: r"
                let newAdder = fn(a, b) {
                    fn(c) { a + b + c };
                };
                let adder = newAdder(1, 2);
                adder(8);"
                    .to_string(),
                expected: Object::INTEGER(11),
            },
            VmTestCase {
                input: r"
                let newAdder = fn(a, b) {
                    let c = a + b;
                    fn(d) { c + d };
                };
                let adder = newAdder(1, 2);
                adder(8);"
                    .to_string(),
                expected: Object::INTEGER(11),
            },
            VmTestCase {
                input: r"
                let newAdderOuter = fn(a, b) {
                    let c = a + b;
                    fn(d) {
                        let e = d + c;
                        fn(f) { e + f; };
                    };
                };
                let newAdderInner = newAdderOuter(1, 2)
                let adder = newAdderInner(3);
                adder(8);"
                    .to_string(),
                expected: Object::INTEGER(14),
            },
            VmTestCase {
                input: r"
                let a = 1;
                let newAdderOuter = fn(b) {
                    fn(c) {
                        fn(d) { a + b + c + d };
                    };
                };
                let newAdderInner = newAdderOuter(2)
                let adder = newAdderInner(3);
                adder(8);"
                    .to_string(),
                expected: Object::INTEGER(14),
            },
            VmTestCase {
                input: r"
                let newClosure = fn(a, b) {
                    let one = fn() { a; };
                    let two = fn() { b; };
                    fn() { one() + two(); };
                };
                let closure = newClosure(9, 90);
                closure();"
                    .to_string(),
                expected: Object::INTEGER(99),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_recursive_functions() {
        let tests = vec![
            VmTestCase {
                input: r"
                let countDown = fn(x) {
                    if (x == 0) {
                        return 0;
                    } else {
                        countDown(x - 1);
                    }
                };
                countDown(1);"
                    .to_string(),
                expected: Object::INTEGER(0),
            },
            VmTestCase {
                input: r"
                let countDown = fn(x) {
                    if (x == 0) {
                        return 0;
                    } else {
                        countDown(x - 1);
                    }
                };
                let wrapper = fn() {
                    countDown(1);
                };
                wrapper();"
                    .to_string(),
                expected: Object::INTEGER(0),
            },
            VmTestCase {
                input: r"
                let wrapper = fn() {
                    let countDown = fn(x) {
                        if (x == 0) {
                            return 0;
                        } else {
                            countDown(x - 1);
                        }
                    };
                    countDown(1);
                };
                wrapper();"
                    .to_string(),
                expected: Object::INTEGER(0),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_recursive_fibonacci() {
        let tests = vec![VmTestCase {
            input: r"
                let fibonacci = fn(x) {
                    if (x == 0) {
                        return 0;
                    } else {
                        if (x == 1) {
                            return 1;
                        } else {
                            fibonacci(x - 1) + fibonacci(x - 2);
                        }
                    }
                };
                fibonacci(15);"
                .to_string(),
            expected: Object::INTEGER(610),
        }];

        run_vm_tests(tests);
    }
}
