use crate::{formatter::formatter::Formatter, lexer::lexer::Lexer, parser::parser::Parser};

#[allow(dead_code)]
fn format(input: &str) -> String {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    Formatter::format_program(program)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_basic_format() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(x, y);
        if (5 < 10) {
            return true;
        } else {return false;
        }
        "#;

        let formatted = format(input);
        let expected = r#"let x = 5;
let y = 10;
let foobar = 838383;
let add = fn (x, y) {
    x + y
};
let result = add(x, y);
if (5 < 10) {
    return true;
} else {
    return false;
}
"#;
        println!("{formatted}");
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_format_arithmetic() {
        let input = r#"
        let x = 5 * 9 + 10;
        let z = 5 * (9 + 10);
        let y = 10 / 5 - 2;
        let yy = 10 / (5 - 2);
        let a = 5 * (9 + 10) / 2;
        let b = 5 * (9 + 10) / (2 + 3);
        let c = (5 * (9 + 10) / (2 + 3)) * 4;
        let d = (5 * (9 + 10) / 2 + 3) * 4;
        let e = [1, 2, 3, 4, 5][1] * 2 + 3;
        let f = {"one": 1, "two": 2}["one"] * 2 + 3;
    "#;
        let formatted = format(input);
        let expected = r#"let x = 5 * 9 + 10;
let z = 5 * (9 + 10);
let y = 10 / 5 - 2;
let yy = 10 / (5 - 2);
let a = 5 * (9 + 10) / 2;
let b = 5 * (9 + 10) / (2 + 3);
let c = 5 * (9 + 10) / (2 + 3) * 4;
let d = (5 * (9 + 10) / 2 + 3) * 4;
let e = [1, 2, 3, 4, 5][1] * 2 + 3;
let f = {"one": 1, "two": 2}["one"] * 2 + 3;
"#;
        println!("{formatted}");
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_prefix_formatting() {
        let input = r#"let x = -5;
        let y = !true;
        let a = -5 + 10;
        let b = !(true == false);
        let b = !(true );
        let c = -(5 + 10);
        let c = -(-5 + 10);
        let c = --(5 + 10);
        let c = -(-(5 + 10));
        let c = ---(5 + 10);
        let d = !!true;
        let d = !(!true);
        "#;

        let formatted = format(input);
        let expected = r#"let x = -5;
let y = !true;
let a = -5 + 10;
let b = !(true == false);
let b = !true;
let c = -(5 + 10);
let c = -(-5 + 10);
let c = --(5 + 10);
let c = --(5 + 10);
let c = ---(5 + 10);
let d = !!true;
let d = !!true;
"#;
        println!("{formatted}");
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_string_has_quotes() {
        let input = r#"let x = "hello";
"#;

        let formatted = format(input);
        let expected = r#"let x = "hello";
"#;

        println!("{formatted}");
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_fibonacci_it_formatting() {
        let input = r#"
            let fibonacci_it= fn(x) {
                if (x < 2){
                    return x;
                }
                let iter = fn (i, table) {
                    if (i > x) {
                        return table[x];
                    } else {
                        let table = push(table, table[i - 1] + table[i - 2]);
                        return iter(i + 1, table);
                    }
                };
                return iter(2, [0,1]);
            };

        let fib = fibonacci_it(20);

        puts(fib);"#;

        let formatted = format(input);

        let expected = r#"let fibonacci_it = fn (x) {
    if (x < 2) {
        return x;
    }
    let iter = fn (i, table) {
        if (i > x) {
            return table[x];
        } else {
            let table = push(table, table[i - 1] + table[i - 2]);
            return iter(i + 1, table);
        }
    };
    return iter(2, [0, 1]);
};
let fib = fibonacci_it(20);
puts(fib);
"#;
        println!("{formatted}");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn format_implicit_return() {
        let inputs = vec![
            r#"
            let fibonacci = fn(x) {
                if (x < 2) {
                    x
                }
                else{
                    fibonacci(x - 1) + fibonacci(x - 2)
                }
            }


        puts(fibonacci(30));
        "#,
            r#"
            let fibonacci = fn(x) {
                puts(x);
                if (x < 2) {
                    x
                }
                else{
                    fibonacci(x - 1) + fibonacci(x - 2)
                }
            }


        puts(fibonacci(30));
        "#,
            r#"
            let fibonacci = fn(x) {
                if (x < 2) {
                    x
                }
                else{
                    return fibonacci(x - 1) + fibonacci(x - 2);
                }
            }"#,
        ];

        let expected_values = vec![
            r#"let fibonacci = fn (x) {
    if (x < 2) {
        x
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
};
puts(fibonacci(30));
"#,
            r#"let fibonacci = fn (x) {
    puts(x);
    if (x < 2) {
        x
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
};
puts(fibonacci(30));
"#,
            r#"let fibonacci = fn (x) {
    if (x < 2) {
        x
    } else {
        return fibonacci(x - 1) + fibonacci(x - 2);
    }
};
"#,
        ];
        for (input, expected) in inputs.iter().zip(expected_values) {
            let formatted = format(input);
            println!("{formatted}");

            assert_eq!(formatted, expected);
        }
    }

    #[test]
    fn format_nested_functions() {
        let input = r#"
            let counter = fn(x) {
                puts(x);
                let count = fn(y) {
                    puts(x + y);
                    x + y
                };
                puts(count(1));
                return count;
            };
            let second_counter = fn (x) {
                puts(x);
                let count = fn(y) {
                    puts(x + y);
                    x + y
                };
                puts(count(1));
                count
            };
            let third = fn (x) {
                let max = fn(y) {
                    if (x < y) {
                        y
                    }
                    else{
                        x
                    }
                };
                count(max(1,x))
            };
            let fourth = fn (x) {
                if (x < 2) {
                    x
                }
                else{
                    let h = fn (y) {
                        if (x < y) {
                            y
                        }
                        else{
                            x
                        }
                    };
                    h(x - 1) + h(x - 2)
                }
            };
            let c = counter(1);
            let d = second_counter(2);
            puts(c(2));
        "#;

        let formatted = format(input);

        let expected = r#"let counter = fn (x) {
    puts(x);
    let count = fn (y) {
        puts(x + y);
        x + y
    };
    puts(count(1));
    return count;
};
let second_counter = fn (x) {
    puts(x);
    let count = fn (y) {
        puts(x + y);
        x + y
    };
    puts(count(1));
    count
};
let third = fn (x) {
    let max = fn (y) {
        if (x < y) {
            y
        } else {
            x
        }
    };
    count(max(1, x))
};
let fourth = fn (x) {
    if (x < 2) {
        x
    } else {
        let h = fn (y) {
            if (x < y) {
                y
            } else {
                x
            }
        };
        h(x - 1) + h(x - 2)
    }
};
let c = counter(1);
let d = second_counter(2);
puts(c(2));
"#;
        println!("{formatted}");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_integer_and_variable_declaration() {
        let input = r#"
            let a = 10;
            let b = 20;
        "#;

        let expected = r#"let a = 10;
let b = 20;
"#;

        assert_eq!(format(input), expected);
    }

    #[test]
    fn test_basic_operators() {
        let input = r#"
            let result = a + b;
            let result = a - b;
            let result = a * b;
            let result = a / b;
            let result = a == b;
            let result = a != b;
            let result = a < b;
            let result = a > b;
            let result = a <= b;
            let result = a >= b;
        "#;

        let expected = r#"let result = a + b;
let result = a - b;
let result = a * b;
let result = a / b;
let result = a == b;
let result = a != b;
let result = a < b;
let result = a > b;
let result = a <= b;
let result = a >= b;
"#;

        println!("{}", format(input));

        assert_eq!(format(input), expected);
    }

    #[test]
    fn test_array_declaration_and_indexing() {
        let input = r#"
            let arr = [1, 2, 3];
            let firstElement = arr[0];
        "#;

        let expected = r#"let arr = [1, 2, 3];
let firstElement = arr[0];
"#;

        assert_eq!(format(input), expected);
    }

    #[test]
    fn test_array_builtin_functions() {
        let input = r#" let arr = [1, 2, 3];
        let length = len(arr);
        let first = first(arr);
        let last = last(arr);
        let restArr = rest(arr);
        let newArr = push(arr, 4);
        "#;

        let expected = r#"let arr = [1, 2, 3];
let length = len(arr);
let first = first(arr);
let last = last(arr);
let restArr = rest(arr);
let newArr = push(arr, 4);
"#;

        assert_eq!(format(input), expected);
    }

    #[test]
    fn test_hash_declaration_and_indexing() {
        let input = r#"
            let hash = {"key1": "value1", "key2": 42};
            let value = hash["key1"];
        "#;

        let expected = r#"let hash = {"key1": "value1", "key2": 42};
let value = hash["key1"];
"#;

        assert_eq!(format(input), expected);
    }

    #[test]
    fn test_function_declaration_and_invocation() {
        let input = r#"
            let add = fn(a, b) {
                return a + b;
            };

            let result = add(5, 10);
        "#;

        let expected = r#"let add = fn (a, b) {
    return a + b;
};
let result = add(5, 10);
"#;

        assert_eq!(format(input), expected);
    }

    #[test]
    fn test_if_else_statement() {
        let input = r#"
            let num = 7;
            if (num < 10) {
                return "Less than 10";
            } else {
                return "10 or greater";
            }
        "#;

        let expected = r#"let num = 7;
if (num < 10) {
    return "Less than 10";
} else {
    return "10 or greater";
}
"#;

        assert_eq!(format(input), expected);
    }

    #[test]
    fn test_function_as_first_class_citizen() {
        let input = r#"
            let double = fn(x) {
                return x * 2;
            };

            let arr = [1, 2, 3, 4];
            let mappedArr = map(arr, double);
        "#;

        let expected = r#"let double = fn (x) {
    return x * 2;
};
let arr = [1, 2, 3, 4];
let mappedArr = map(arr, double);
"#;

        assert_eq!(format(input), expected);
    }

    #[test]
    fn test_shadowing() {
        let input = r#"
            let a = 5;
            let a = 10;
        "#;

        let expected = r#"let a = 5;
let a = 10;
"#;

        assert_eq!(format(input), expected);
    }

    #[test]
    fn test_built_in_function() {
        let input = r#"
            puts("Hello, Monkey!");
            let arr = [1, 2, 3];
            let length = len(arr);
        "#;

        let expected = r#"puts("Hello, Monkey!");
let arr = [1, 2, 3];
let length = len(arr);
"#;

        assert_eq!(format(input), expected);
    }
}
