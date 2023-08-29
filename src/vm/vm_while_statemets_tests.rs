#[allow(clippy::too_many_lines)]
#[cfg(test)]
mod tests {

    use crate::{
        object::Object,
        vm::vm_tests::{run_vm_tests, VmTestCase},
    };

    #[test]
    fn test_while_statements_without_break_or_continue() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let a = 1;
                    while (a < 100){
                        let a = a + 1;
                    }
                    a
                "#
                .to_string(),
                expected: Object::INTEGER(100),
            },
            VmTestCase {
                input: r#"
                    let a = 1;
                    while (a < 0){
                        let a = 100;
                    }
                    a
                    "#
                .to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: r#"
                    let a = 1;
                    while(false){
                        let a = 100;
                    }
                    a
                    "#
                .to_string(),
                expected: Object::INTEGER(1),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_while_clean_up() {
        // This tests makes sure that a while statement clears the stack correctly (which is
        // different from the conditional behavior)
        let tests = vec![VmTestCase {
            input: r#"
                    let a = 0;
                    while (a < 10000){
                        let a = a + 1;
                        puts(a);
                    }
                    a
                    "#
            .to_string(),
            expected: Object::INTEGER(10000),
        }];

        run_vm_tests(tests);
    }

    #[test]
    fn test_break_from_while() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let a = 0;
                    while (a < 10) {
                        if (a == 5) {
                            break;
                        } 
                        let a = a + 1;
                    };
                    a"#
                .to_string(),
                expected: Object::INTEGER(5),
            },
            VmTestCase {
                input: r#"
            let a = 0;
            let c = 0;
             while (a < 10) {
                 let b = 0;
                 while (b < 10) {
                     if (b == 5) {
                         break;
                     }
                     let b = b + 1;
                     let c = c + 1;
                 }
                 let a = a + 1; 
             };
             c"#
                .to_string(),
                expected: Object::INTEGER(50),
            },
            VmTestCase {
                input: r#"
            let a = 0;
            let c = 0;
             while (a < 10) {
                 if (a == 5) {
                     break;
                 }

                 let b = 0;
                 while (b < 10) {
                     if (b == 5) {
                         break;
                     }
                     let b = b + 1;
                     let c = c + 1;
                 }
                 let a = a + 1; 
             };
             c"#
                .to_string(),
                expected: Object::INTEGER(25),
            },
            // The next tests will take care of the possible interference between the break and a function
            VmTestCase {
                input: r#"
                let f = fn (a) {
                    let c = 0;
                    while (a < 10) {
                        if (a == 5) {
                            break;
                        }

                        let b = 0;
                        while (b < 10) {
                            if (b == 5) {
                                break;
                            }
                            let b = b + 1;
                            let c = c + 1;
                        }
                        let a = a + 1; 
                    }
                    c
                };
                f(0)"#
                    .to_string(),
                expected: Object::INTEGER(25),
            },
            VmTestCase {
                input: r#"
                let a = 0;
                let c = 0;
                while (a < 10) {
                    if (a == 5) {
                            break;
                    }

                    let f = fn () {
                        let c = 0;
                        let b = 0;
                        while (b < 10) {
                            if (b == 5) {
                                break;
                            }
                            let b = b + 1;
                            let c = c + 1;
                        }
                        c
                    }

                    let a = a + 1; 
                    let c = c + f();
                };
                c"#
                .to_string(),
                expected: Object::INTEGER(25),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_continue_from_while() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let a = 0;
                    let c = 0;
                    while (a < 10) {
                        let a = a + 1;
                        if (a == 5) {
                            let c  = c + 2;
                            continue;
                        } 
                        let c = c + 1;
                    };
                    c"#
                .to_string(),
                expected: Object::INTEGER(11),
            },
            VmTestCase {
                input: r#"
            let a = 0;
            let c = 0;
             while (a < 10) {
                 let b = 0;
                 while (b < 10) {
                     let b = b + 1;
                     if (b == 5) {
                         let c = c + 3;
                         continue;
                     }
                     let c = c + 1;
                 }
                 let a = a + 1; 
             };
             c"#
                .to_string(),
                expected: Object::INTEGER(120),
            },
            // The next tests will take care of the possible interference between the continue and a function
            VmTestCase {
                input: r#"
                let f = fn (a) {
                    let c = 0;
                    while (a < 10) {
                        let b = 0;
                        while (b < 10) {
                            let b = b + 1;
                            if (b == 5) {
                                let c = c + 3;
                                continue;
                            }
                            let c = c + 1;
                        }
                        let a = a + 1; 
                    }
                    c
                };
                f(0)"#
                    .to_string(),
                expected: Object::INTEGER(120),
            },
            VmTestCase {
                input: r#"
                let a = 0;
                let c = 0;
                while (a < 10) {
                    let f = fn () {
                        let c = 0;
                        let b = 0;
                        while (b < 10) {
                            let b = b + 1;
                            if (b == 5) {
                                let c = c + 3;
                                continue;
                            }
                            let c = c + 1;
                        }
                        c
                    }

                    let a = a + 1; 
                    let c = c + f();
                };
                c"#
                .to_string(),
                expected: Object::INTEGER(120),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_continue_and_break_in_while() {
        let tests = vec![VmTestCase {
            input: r#"
                let a = 0;
                let c = 0;
                while (a < 10) {
                    let a = a + 1; 
                    if (a == 5) {
                        let c = c + 3;
                        continue;
                    }
                    if (a == 7) {
                        break;
                    }
                    let c = c + 1;
                }
                c"#
            .to_string(),
            expected: Object::INTEGER(8),
        }];
        run_vm_tests(tests);
    }
}
