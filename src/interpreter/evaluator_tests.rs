#[cfg(test)]
mod tests {

    use crate::{interpreter::evaluator::Evaluator, lexer::Lexer, object::Object, parser::Parser};
    use std::collections::HashMap;

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = vec![
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 <= 2", true),
            ("1 >= 2", false),
            ("1 <= 1", true),
            ("1 >= 1", true),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            //
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            //
            ("false && true", false),
            ("true && false", false),
            ("false && false", false),
            ("true && true", true),
            //
            ("false || true", true),
            ("true || false", true),
            ("false || false", false),
            ("true || true", true),
            //
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_boolean_object(evaluated, expected);
        }
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_boolean_object(evaluated, expected);
        }
    }

    #[test]
    fn test_if_else_expression() {
        let tests = vec![
            ("if (true) { 10 }", Some(10)),
            ("if (false) { 10 }", None),
            ("if (1) { 10 }", Some(10)),
            ("if (1 < 2) { 10 }", Some(10)),
            ("if (1 > 2) { 10 }", None),
            ("if (1 > 2) { 10 } else { 20 }", Some(20)),
            ("if (1 < 2) { 10 } else { 20 }", Some(10)),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            if let Some(expected) = expected {
                test_integer_object(evaluated, expected);
            } else {
                test_null_object(evaluated);
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            ("if (10 > 1) { return 10; }", 10),
            ("if (10 > 1) { if (10 > 1) { return 10; } return 1; }", 10),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -true"),
            ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                r#"
                if (10 > 1) {
                    if (10 > 1) {
                        return true + false;
                    }
                    return 1;
                }"#,
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "identifier not found: foobar"),
            (r#""Hello" - "World""#, "unknown operator: STRING - STRING"),
            (
                r#"{"name": "Monkey"}[fn(x) { x }];"#,
                "unusable as hash key: FUNCTION",
            ),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_error_object(evaluated, expected.to_string());
        }
    }

    #[test]
    fn test_let_stateemtns() {
        let tests = vec![
            ("let a = 5; a;", Some(5)),
            ("let a = 5 * 5; a;", Some(25)),
            ("let a = 5; let b = a; b;", Some(5)),
            ("let a = 5; let b = a; let c = a + b + 5; c;", Some(15)),
            ("let a = 5;", None),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            match expected {
                Some(expected) => test_integer_object(evaluated, expected),
                None => test_null_object(evaluated),
            }
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2; };";

        let evaluated = test_eval(input);

        match evaluated {
            Object::FUNCTION(x) => {
                assert_eq!(x.parameters.len(), 1);
                assert_eq!(x.parameters[0].to_string(), "x");
                assert_eq!(x.body.to_string(), "(x + 2)\n");
            }
            _ => panic!("The object is not a function"),
        }
    }

    #[test]
    fn test_function_application() {
        let tests = vec![
            ("let identity = fn(x) { x; }; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 11);", 16),
            (
                "let add = fn(x, y) { x + y; }; add(5 + 5, add(10, 10));",
                30,
            ),
            ("fn(x) { x; }(5)", 5),
        ];
        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
        }
    }

    #[test]
    fn test_closures() {
        let input = r#"
        let newAdder = fn(x) {
            fn(y) { x + y };
        };

        let addTwo = newAdder(2);
        addTwo(2);"#;

        test_integer_object(test_eval(input), 4);
    }

    #[test]
    fn test_string_literal() {
        let input = "\"Hello World!\"";

        let evaluated = test_eval(input);

        test_string_object(evaluated, "Hello World!".to_string());
    }

    #[test]
    fn test_string_concatenationm() {
        let input = "\"Hello\" + \" \" + \"World!\"";

        let evaluated = test_eval(input);

        test_string_object(evaluated, "Hello World!".to_string());
    }

    #[test]
    fn test_builttin_len_function() {
        let tests_striung = vec![
            (r#"len("")"#, 0),
            (r#"len("four")"#, 4),
            (r#"len("hello world")"#, 11),
            (r#"len([1,2,3,4,5])"#, 5),
        ];

        for (input, expected) in tests_striung {
            test_integer_object(test_eval(input), expected);
        }
    }

    #[test]
    fn test_builttin_len_function_errors() {
        let tests_striung = vec![
            (r#"len(1)"#, "argument to `len` not supported, got INTEGER"),
            (
                r#"len("one", "two")"#,
                "wrong number of arguments. got=2, want=1",
            ),
        ];

        for (input, expected) in tests_striung {
            test_error_object(test_eval(input), expected.to_string());
        }
    }

    #[test]
    fn test_array_literals() {
        let input = "[1, 2 * 2, 3 + 3]";

        let evaluated = test_eval(input);

        match evaluated {
            Object::ARRAY(x) => {
                assert_eq!(x.len(), 3);
                test_integer_object(x[0].clone(), 1);
                test_integer_object(x[1].clone(), 4);
                test_integer_object(x[2].clone(), 6);
            }
            _ => panic!("The object is not an array"),
        }
    }

    #[test]
    fn test_array_index_expression() {
        let tests = vec![
            ("[1, 2, 3][0]", Some(1)),
            ("[1, 2, 3][1]", Some(2)),
            ("[1, 2, 3][2]", Some(3)),
            ("let i = 0; [1][i];", Some(1)),
            ("[1, 2, 3][1 + 1];", Some(3)),
            ("let myArray = [1, 2, 3]; myArray[2];", Some(3)),
            (
                "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
                Some(6),
            ),
            (
                "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
                Some(2),
            ),
            ("[1, 2, 3][3]", None),
            ("[1, 2, 3][-1]", None),
        ];

        for (input, expected) in tests {
            match expected {
                Some(x) => test_integer_object(test_eval(input), x),
                None => test_null_object(test_eval(input)),
            }
        }
    }

    #[test]
    fn test_first_function() {
        let tests = vec![
            ("first([1, 2, 3])", Some(1)),
            ("first([1])", Some(1)),
            ("first([])", None),
            ("first(1)", None),
            ("first([1, 2, 3], [4, 5, 6])", None),
        ];

        for (input, expected) in tests {
            println!("{input}");
            match expected {
                Some(x) => test_integer_object(test_eval(input), x),
                None => test_null_object(test_eval(input)),
            }
        }
    }

    #[test]
    fn test_last_function() {
        let tests = vec![
            ("last([1, 2, 3])", Some(3)),
            ("last([1])", Some(1)),
            ("last([])", None),
            ("last(1)", None),
            ("last([1, 2, 3], [4, 5, 6])", None),
        ];

        for (input, expected) in tests {
            println!("{input}");
            match expected {
                Some(x) => test_integer_object(test_eval(input), x),
                None => test_null_object(test_eval(input)),
            }
        }
    }

    #[test]
    fn test_rest_function() {
        let tests = vec![
            ("rest([1, 2, 3])", Some(vec![2, 3])),
            ("rest([1])", Some(Vec::new())),
            ("rest([])", None),
            ("rest(1)", None),
            ("rest([1, 2, 3], [4, 5, 6])", None),
        ];

        for (input, expected) in tests {
            println!("{input}");
            match expected {
                Some(x) => {
                    let evaluated = test_eval(input);
                    test_array_object(evaluated, x);
                }
                None => test_null_object(test_eval(input)),
            }
        }
    }

    #[test]
    fn test_push_function() {
        let tests = vec![
            ("push([], 1)", Some(vec![1])),
            ("push([1], 2)", Some(vec![1, 2])),
            ("push([1,2], 3)", Some(vec![1, 2, 3])),
            ("push(1, 1)", None),
            ("push([1,2], 3, 4)", None),
        ];

        for (input, expected) in tests {
            println!("{input}");
            match expected {
                Some(x) => test_array_object(test_eval(input), x),
                None => test_null_object(test_eval(input)),
            }
        }
    }

    #[test]
    fn test_array_functions_together() {
        let input = r#"
        let map = fn(arr, f) {
            let iter = fn(arr, accumulated) {
                if (len(arr) == 0) {
                    accumulated
                } else {
                    iter(rest(arr), push(accumulated, f(first(arr))));
                }
            };
            iter(arr, []);
        };
        let a = [1, 2, 3, 4];
        let double = fn(x) { x * 2 };
        map(a, double);
        "#;

        let expected = vec![2, 4, 6, 8];

        test_array_object(test_eval(input), expected);
    }

    #[test]
    fn test_evaluate_hash_literals() {
        let input = r#"
        let two = "two";
        {
            "one": 10 - 9,
            two: 1 + 1,
            "thr" + "ee": 6 / 2,
            4: 4,
            true: 5,
            false: 6
        }
        "#;

        let mut expected = HashMap::new();
        expected.insert(Object::STRING("one".to_string()), Object::INTEGER(1));
        expected.insert(Object::STRING("two".to_string()), Object::INTEGER(2));
        expected.insert(Object::STRING("three".to_string()), Object::INTEGER(3));
        expected.insert(Object::INTEGER(4), Object::INTEGER(4));
        expected.insert(Object::BOOLEAN(true), Object::INTEGER(5));
        expected.insert(Object::BOOLEAN(false), Object::INTEGER(6));

        let evaluated = test_eval(input);
        match evaluated {
            Object::HASHMAP(hash) => {
                assert_eq!(hash.len(), expected.len());

                for (expected_key, expected_value) in expected {
                    match hash.get(&expected_key) {
                        Some(value) => assert_eq!(value, &expected_value),
                        None => panic!("No pair for given key in Pairs"),
                    }
                }
            }
            _ => panic!("The object is not a hash"),
        }
    }

    #[test]
    fn test_hash_index_expressions() {
        let tests = vec![
            (r#"{"foo": 5}["foo"]"#, Some(5)),
            (r#"{"foo": 5}["bar"]"#, None),
            (r#"let key = "foo"; {"foo": 5}[key]"#, Some(5)),
            (r#"{}["foo"]"#, None),
            (r#"{5: 5}[5]"#, Some(5)),
            (r#"{true: 5}[true]"#, Some(5)),
            (r#"{false: 5}[false]"#, Some(5)),
        ];

        for (input, expected) in tests {
            println!("{input}");
            match expected {
                Some(x) => test_integer_object(test_eval(input), x),
                None => test_null_object(test_eval(input)),
            }
        }
    }

    #[test]
    fn test_while_statements() {
        let tests = vec![
            ("let a = 0; while (a < 10) { let a = a + 1; }; a", Some(10)),
            (
                "let a = 100; while (a < 10) { let a = a + 1; }; a",
                Some(100),
            ),
            ("while (false) { 1 }", None),
        ];

        for (input, expected) in tests {
            println!("{input}");
            match expected {
                Some(x) => test_integer_object(test_eval(input), x),
                None => test_null_object(test_eval(input)),
            }
        }
    }

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new();
        evaluator.eval(program)
    }

    fn test_integer_object(object: Object, expected: i64) {
        match object {
            Object::INTEGER(x) => assert_eq!(x, expected),
            x => panic!("The object is not an integer, it is {:#?}", x),
        }
    }

    fn test_boolean_object(object: Object, expected: bool) {
        match object {
            Object::BOOLEAN(x) => assert_eq!(x, expected),
            _ => panic!("The object is not a boolean"),
        }
    }

    fn test_null_object(object: Object) {
        match object {
            Object::NULL | Object::ERROR(_) => (),

            _ => panic!("The object is not null"),
        }
    }

    fn test_error_object(object: Object, expected: String) {
        match object {
            Object::ERROR(x) => assert_eq!(x, expected),
            _ => panic!("The object is not an  error"),
        }
    }

    fn test_string_object(object: Object, expected: String) {
        match object {
            Object::STRING(s) => assert_eq!(format!("{s}"), expected),
            _ => panic!("The object is not an string"),
        }
    }

    fn test_array_object(object: Object, expected: Vec<i64>) {
        match object {
            Object::ARRAY(x) => {
                assert_eq!(x.len(), expected.len());
                for (i, v) in x.iter().enumerate() {
                    test_integer_object(v.clone(), expected[i]);
                }
            }
            _ => panic!("The object is not an array"),
        }
    }
}
