#[allow(clippy::too_many_lines)]
#[cfg(test)]
mod tests {

    use crate::{
        object::Object,
        vm::test_utils::{run_vm_tests, VmTestCase},
    };

    #[test]
    fn test_use_same_varaible_multiple_times() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let a = 1;
                    let b = a;
                    let c = a + b + 1;
                    c"#
                .to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r#"
                    let a = 1;
                    let b = a;
                    let c = a + b + 1;
                    let a = 2;
                    let b = a;
                    let c = a + b + 1;
                    c"#
                .to_string(),
                expected: Object::INTEGER(5),
            },
            VmTestCase {
                input: r#"
                    let a = "hello";
                    let b = "world";
                    let c = a + b;
                    let d = a + b + c;
                    d"#
                .to_string(),
                expected: Object::STRING("helloworldhelloworld".to_string()),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_array_multiple_ussage() {
        let tests = vec![
            VmTestCase {
                input: r#"
                let array = [1,2,3];
                let value =  array[1] + array[2];
                value"#
                    .to_string(),
                expected: Object::INTEGER(5),
            },
            VmTestCase {
                input: r#"
                let array = [1,2,3];
                let array = push(array, 4);
                array"#
                    .to_string(),
                expected: Object::ARRAY(vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::INTEGER(3),
                    Object::INTEGER(4),
                ]),
            },
        ];
        run_vm_tests(tests);
    }

    #[test]
    fn test_shadowing_same_type() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let x = 4;
                    let x = 5;
                    x"#
                .to_string(),
                expected: Object::INTEGER(5),
            },
            VmTestCase {
                input: r#"
                    let x = [1,2,3];
                    let x = [4,5,6];
                    x"#
                .to_string(),
                expected: Object::ARRAY(vec![
                    Object::INTEGER(4),
                    Object::INTEGER(5),
                    Object::INTEGER(6),
                ]),
            },
            VmTestCase {
                input: r#"
                    let x = fn() { 1 };
                    let x = fn() { 2 };
                    x()"#
                    .to_string(),
                expected: Object::INTEGER(2),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_shadowing_with_new_type() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let x = 4;
                    let x = "string";
                    x"#
                .to_string(),
                expected: Object::STRING("string".to_string()),
            },
            VmTestCase {
                input: r#"
                    let x = "string";
                    let x = fn() { 1 };
                    x()"#
                    .to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: r#"
                    let x = fn() { 1 };
                    let x = 5,
                    x"#
                .to_string(),
                expected: Object::INTEGER(5),
            },
            VmTestCase {
                input: r#"
                    let x = 5;
                    let x = [1,2,3];
                    x"#
                .to_string(),
                expected: Object::ARRAY(vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::INTEGER(3),
                ]),
            },
            VmTestCase {
                input: r#"
                    let x = [1,2,3];
                    let x = 5;
                    x"#
                .to_string(),
                expected: Object::INTEGER(5),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_shadowing_using_previous_value() {
        let tests = vec![
            VmTestCase {
                input: r#"
                    let a = 1;
                    let b = a * a + 2
                    b"#
                .to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: r#"
                    let a = 1;
                    let a = a + 1;
                    a"#
                .to_string(),
                expected: Object::INTEGER(2),
            },
            VmTestCase {
                input: r#"
                    let a = fn() { 
                        let a = 1;
                        a
                    };
                    a()
                    "#
                .to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: r#"
                    let f = fn(a){
                        let a = 1;
                        let a = a + 1;
                        a
                    };
                    f(1)
                    "#
                .to_string(),
                expected: Object::INTEGER(2),
            },
            VmTestCase {
                input: r#"
                    let f = fn(){
                        let a = 1;
                        let h = fn(){
                            let a = 2;
                            a
                        };
                        h() + a
                    };
                    f()
                    "#
                .to_string(),
                expected: Object::INTEGER(3),
            },
            // Addition of a global variable a with 10 as its value
            VmTestCase {
                input: r#"
                    let a = 10;
                    let a = fn() { 
                        let a = 1;
                        a
                    };
                    a()
                    "#
                .to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: r#"
                    let a = 10;
                    let f = fn(a){
                        let a = 1;
                        a
                    };
                    f(1) + a
                    "#
                .to_string(),
                expected: Object::INTEGER(11),
            },
            VmTestCase {
                input: r#"
                    let a = 10;
                    let f = fn(a){
                        let a = 1;
                        let a = a + 1;
                        a
                    };
                    f(1) + a
                    "#
                .to_string(),
                expected: Object::INTEGER(12),
            },
            VmTestCase {
                input: r#"
                    let a = 10;
                    let f = fn(){
                        let h = fn(){
                            let a = 2;
                            a
                        };
                        h()
                    };
                    f() + a
                    "#
                .to_string(),
                expected: Object::INTEGER(12),
            },
            VmTestCase {
                input: r#"
                    let a = 10;
                    let f = fn(){
                        let a = 1;
                        let a = a + 1;
                        let h = fn(){
                            let a = 2;
                            a
                        };
                        h() + a
                    };
                    f() + a
                    "#
                .to_string(),
                expected: Object::INTEGER(14),
            },
        ];

        run_vm_tests(tests);
    }
}
