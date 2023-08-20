#[cfg(test)]
mod tests {

    use crate::{
        lexer::{token::Token, Lexer},
        parser::{
            ast::{
                BlockStatement, Conditional, ControlFlow, Expression, FunctionCall, Identifier,
                InfixOperator, LetStatement, Primitive, Program, ReturnStatement, Statement,
                WhileStatement,
            },
            Parser,
        },
    };

    #[test]
    fn test_let_statements() {
        let input = r#"let x = 5;
        let y = true;
        let foobar = y;
        "#;

        let program = generate_program(input);
        let expected_statemets = vec![
            Statement::Let(LetStatement {
                name: Identifier {
                    token: Token::Ident("x".to_string()),
                    value: "x".to_string(),
                },
                value: Expression::Primitive(Primitive::IntegerLiteral(5)),
            }),
            Statement::Let(LetStatement {
                name: Identifier {
                    token: Token::Ident("y".to_string()),
                    value: "y".to_string(),
                },
                value: Expression::Primitive(Primitive::BooleanLiteral(true)),
            }),
            Statement::Let(LetStatement {
                name: Identifier {
                    token: Token::Ident("foobar".to_string()),
                    value: "foobar".to_string(),
                },
                value: Expression::Identifier(Identifier {
                    token: Token::Ident("y".to_string()),
                    value: "y".to_string(),
                }),
            }),
        ];

        assert_eq!(program.statements.len(), expected_statemets.len());

        for (i, expected) in expected_statemets.iter().enumerate() {
            println!("{} | {} | {} ", i, expected, program.statements[i]);
            assert_eq!(program.statements[i], *expected);
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
        return 5;
        return true;
        return y;
        "#;

        let program = generate_program(input);
        let expected = vec![
            Statement::Return(ReturnStatement {
                return_value: Expression::Primitive(Primitive::IntegerLiteral(5)),
            }),
            Statement::Return(ReturnStatement {
                return_value: Expression::Primitive(Primitive::BooleanLiteral(true)),
            }),
            Statement::Return(ReturnStatement {
                return_value: Expression::Identifier(Identifier {
                    token: Token::Ident("y".to_string()),
                    value: "y".to_string(),
                }),
            }),
        ];

        assert_eq!(program.statements.len(), 3);

        for (i, expected) in expected.iter().enumerate() {
            assert_eq!(program.statements[i], *expected);
        }
    }

    fn check_parse_errors(parser: &Parser) {
        let len = parser.errors.len();

        if len > 0 {
            println!("Parser has {} errors", parser.errors.len());
            println!("Parser errors: {:?}", parser.errors);
        }
        assert_eq!(len, 0);
    }

    #[test]
    fn test_errors() {
        let input = r#"
        let x 5;
        let = 10;
        let 838383;
        let x = 838383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        parser.parse_program();

        assert_ne!(parser.errors.len(), 0);
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        assert_eq!(
            statement,
            &Statement::Expression(Expression::Identifier(Identifier {
                token: Token::Ident("foobar".to_string()),
                value: "foobar".to_string(),
            }))
        );
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        assert_eq!(
            statement,
            &Statement::Expression(Expression::Primitive(Primitive::IntegerLiteral(5)))
        );
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let tests = vec![
            ("!5", "!", "5"),
            ("-15", "-", "15"),
            ("!true;", "!", "true"),
            ("!false;", "!", "false"),
        ];

        for (input, operator, value) in tests {
            let program = generate_program(input);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => check_prefix_expression(exp, operator, value),
                _ => panic!("It is not an expression statement"),
            };
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let tests = vec![
            ("5 + 5;", "5", "+", "5"),
            ("5 - 5;", "5", "-", "5"),
            ("5 * 5;", "5", "*", "5"),
            ("5 / 5;", "5", "/", "5"),
            ("5 > 5;", "5", ">", "5"),
            ("5 >= 5;", "5", ">=", "5"),
            ("5 < 5;", "5", "<", "5"),
            ("5 <= 5;", "5", "<=", "5"),
            ("5 == 5;", "5", "==", "5"),
            ("5 != 5;", "5", "!=", "5"),
            ("true == true", "true", "==", "true"),
            ("true != false", "true", "!=", "false"),
            ("false == false", "false", "==", "false"),
            ("false && true", "false", "&&", "true"),
            ("true || false", "true", "||", "false"),
        ];

        for (input, left, operator, right) in tests {
            let program = generate_program(input);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => check_infix_expression(exp, left, operator, right),
                _ => panic!("It is not an expression statement"),
            }
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let test = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)\n((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
            (
                "a * [1, 2, 3, 4][b * c] * d",
                "((a * ([1, 2, 3, 4][(b * c)])) * d)",
            ),
            (
                "add(a * b[2], b[1], 2 * [1, 2][1])",
                "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
            ),
        ];

        for (input, expected) in test {
            let program = generate_program(input);
            print!("{program}");
            assert_ne!(program.statements.len(), 0);
            assert_eq!(program.to_string(), format!("{expected}\n"));
        }
    }

    #[test]
    fn test_boolean_expression() {
        let tests = vec![("true;", true), ("false;", false)];

        for (input, expected) in tests {
            let program = generate_program(input);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => check_primitive_literal(exp, &expected.to_string()),
                _ => panic!("It is not an expression statement"),
            }
        }
    }

    #[test]
    fn test_if_statement() {
        let (input, condition, consequence, alternative) = ("if (x < y) { x }", "x < y", "x", None);
        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => {
                check_conditional_expression(exp, condition, consequence, alternative);
            }
            _ => panic!("It is not an expression statement"),
        }
    }

    #[test]
    fn test_if_else_statement() {
        let (input, condition, consequence, alternative) =
            ("if (x < y) { x } else {y}", "x < y", "x", Some("y"));
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parse_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => {
                check_conditional_expression(exp, condition, consequence, alternative);
            }
            _ => panic!("It is not an expression statement"),
        }
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn(x, y) { x + y; }";
        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => check_function_literal(exp, vec!["x", "y"], "(x + y)"),
            _ => panic!("It is not an expression statement"),
        }
    }

    #[test]
    fn test_parse_funtion_arguments() {
        let tests = vec![
            ("fn() {}", Vec::new()),
            ("fn(x) {}", vec!["x"]),
            ("fn(x,y,z) {}", vec!["x", "y", "z"]),
        ];

        for (input, expected) in tests {
            let program = generate_program(input);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => check_function_literal(exp, expected, ""),
                _ => panic!("It is not an expression statement"),
            }
        }
    }

    #[test]
    fn test_function_call_parsing() {
        let (input, name, argumnets) = (
            "add(1, 2 * 3, 4 + 5);",
            "add",
            vec!["1", "(2 * 3)", "(4 + 5)"],
        );

        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => check_function_call(exp, name, argumnets),
            _ => panic!("It is not an expression statement"),
        }
    }

    #[test]
    fn test_function_call_parameter_parsing() {
        let tests = vec![
            ("add();", "add", vec![]),
            ("add(1);", "add", vec!["1"]),
            (
                "add(1, 2 * 3, 4 + 5);",
                "add",
                vec!["1", "(2 * 3)", "(4 + 5)"],
            ),
        ];

        for (input, name, argumnets) in tests {
            let program = generate_program(input);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => check_function_call(exp, name, argumnets),
                _ => panic!("It is not an expression statement"),
            }
        }
    }

    #[test]
    fn test_string_literal_expression() {
        let input = "\"hello world\";";

        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => check_primitive_literal(exp, "hello world"),
            _ => panic!("It is not an expression statement"),
        }
    }

    #[test]
    fn test_array_literal() {
        let input = "[1,2*2,3+3]";

        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);
        let expressions = match &program.statements[0] {
            Statement::Expression(exp) => match exp {
                Expression::ArrayLiteral(a) => &a.elements,
                _ => panic!("It is not an array literal"),
            },
            _ => panic!("It is not an expression statement"),
        };

        assert_eq!(expressions.len(), 3);
        check_primitive_literal(&expressions[0], "1");
        check_infix_expression(&expressions[1], "2", "*", "2");
        check_infix_expression(&expressions[2], "3", "+", "3");
    }

    #[test]
    fn test_parsing_index_expression_complete() {
        let input = "myArray[1+1]";

        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => match exp {
                Expression::IndexExpression(i) => {
                    assert_eq!(i.left.to_string(), "myArray");
                    check_infix_expression(&i.index, "1", "+", "1");
                }
                _ => panic!("It is not an index expression"),
            },
            _ => panic!("It is not an expression statement"),
        }
    }

    #[test]
    fn test_parsing_index_expression_string_conversion() {
        let tests = vec![
            ("myArray[1]", "myArray", "1"),
            ("myArray[\"hello\"]", "myArray", "\"hello\""),
            ("[1,2,3,4][2]", "[1, 2, 3, 4]", "2"),
            ("test()[call()]", "test()", "call()"),
        ];

        for (input, left, index) in tests {
            let program = generate_program(input);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => check_index_expression(exp, left, index),

                _ => panic!("It is not an expression statement"),
            }
        }
    }

    #[test]
    fn test_parsing_hash_map_literal_string_keys() {
        let input = "{\"one\": 1, \"two\": 2, \"three\": 3}";

        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => match exp {
                Expression::HashMapLiteral(h) => {
                    assert_eq!(h.pairs.len(), 3);
                    let expected = vec![("one", "1"), ("two", "2"), ("three", "3")];
                    for (i, (key, value)) in expected.iter().enumerate() {
                        let pair = h.pairs.get(i).unwrap();
                        check_primitive_literal(&pair.0, key);
                        check_primitive_literal(&pair.1, value);
                    }
                }
                _ => panic!("It is not an hash literal"),
            },
            _ => panic!("It is not an expression statement"),
        }
    }

    #[test]
    fn test_parsing_empty_hash_map() {
        let input = "{}";

        let program = generate_program(input);
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => match exp {
                Expression::HashMapLiteral(h) => {
                    assert_eq!(h.pairs.len(), 0);
                }
                _ => panic!("It is not an hash literal"),
            },
            _ => panic!("It is not an expression statement"),
        }
    }

    #[test]
    fn test_parsing_hash_map_literal_integer_values() {
        let input = "{\"one\": 1 + 34, \"two\": 2/5, \"three\": 3-1}";

        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => match exp {
                Expression::HashMapLiteral(h) => {
                    assert_eq!(h.pairs.len(), 3);
                    let expected = vec![
                        ("\"one\"", "(1 + 34)"),
                        ("\"two\"", "(2 / 5)"),
                        ("\"three\"", "(3 - 1)"),
                    ];
                    for (i, (key, value)) in expected.iter().enumerate() {
                        let pair = h.pairs.get(i).unwrap();
                        assert_eq!(pair.0.to_string(), **key);
                        assert_eq!(pair.1.to_string(), **value);
                    }
                }
                _ => panic!("It is not an hash literal"),
            },
            _ => panic!("It is not an expression statement"),
        }
    }

    #[test]
    fn test_parsing_hash_map_literal_mixed_keys() {
        let input = "{1:true, 2: \"Hi\", \"three\": 3-1}";

        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => match exp {
                Expression::HashMapLiteral(h) => {
                    assert_eq!(h.pairs.len(), 3);
                    let expected = vec![("1", "true"), ("2", "\"Hi\""), ("\"three\"", "(3 - 1)")];
                    for (i, (key, value)) in expected.iter().enumerate() {
                        let pair = h.pairs.get(i).unwrap();
                        assert_eq!(pair.0.to_string(), **key);
                        assert_eq!(pair.1.to_string(), **value);
                    }
                }
                _ => panic!("It is not an hash literal"),
            },
            _ => panic!("It is not an expression statement"),
        }
    }

    #[test]
    fn test_parsing_function_literal_with_name() {
        let input = "let myFunction = fn(){};";

        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);
        match program.statements[0].clone() {
            Statement::Let(l) => match l.value {
                Expression::FunctionLiteral(f) => {
                    assert_eq!(f.name, Some("myFunction".to_string()));
                }
                _ => panic!("It is not a function literal"),
            },
            _ => panic!("It is not a let statement"),
        }
    }

    #[test]
    fn test_parsing_function_literal_without_name() {
        let input = "fn(){};";

        let program = generate_program(input);

        assert_eq!(program.statements.len(), 1);
        match program.statements[0].clone() {
            Statement::Expression(exp) => match exp {
                Expression::FunctionLiteral(f) => {
                    assert!(f.name.is_none());
                }
                _ => panic!("It is not a function literal"),
            },
            _ => panic!("It is not an expression"),
        }
    }

    #[test]
    fn test_parsing_while_statements() {
        let input = "while(x < 3){
            let x = x + 3;
            puts(x);
        }";

        let expected = WhileStatement {
            condition: Expression::Infix(InfixOperator {
                token: Token::LT,
                left: Box::new(Expression::Identifier(Identifier {
                    token: Token::Ident("x".to_string()),
                    value: "x".to_string(),
                })),
                right: Box::new(Expression::Primitive(Primitive::IntegerLiteral(3))),
            }),
            body: BlockStatement {
                statements: vec![
                    Statement::Let(LetStatement {
                        name: Identifier {
                            token: Token::Ident("x".to_string()),
                            value: "x".to_string(),
                        },
                        value: Expression::Infix(InfixOperator {
                            token: Token::Plus,
                            left: Box::new(Expression::Identifier(Identifier {
                                token: Token::Ident("x".to_string()),
                                value: "x".to_string(),
                            })),
                            right: Box::new(Expression::Primitive(Primitive::IntegerLiteral(3))),
                        }),
                    }),
                    Statement::Expression(Expression::FunctionCall(FunctionCall {
                        function: Box::new(Expression::Identifier(Identifier {
                            token: Token::Ident("puts".to_string()),
                            value: "puts".to_string(),
                        })),
                        arguments: vec![Expression::Identifier(Identifier {
                            token: Token::Ident("x".to_string()),
                            value: "x".to_string(),
                        })],
                    })),
                ],
            },
        };

        println!("Input:\n{input}");
        let program = generate_program(input);
        println!("Parsed:\n{program}");

        assert_eq!(program.statements.len(), 1);

        match program.statements[0].clone() {
            Statement::While(smt) => {
                assert_eq!(smt, expected);
            }
            _ => panic!("It is not an expression"),
        }
    }

    #[test]
    fn test_parse_while_control_flow_statements() {
        let input = "while(x < 3){
        if (x == 2){
            break;
        } else {
            continue;
        }
    }";

        let expected = WhileStatement {
            condition: Expression::Infix(InfixOperator {
                token: Token::LT,
                left: Box::new(Expression::Identifier(Identifier {
                    token: Token::Ident("x".to_string()),
                    value: "x".to_string(),
                })),
                right: Box::new(Expression::Primitive(Primitive::IntegerLiteral(3))),
            }),
            body: BlockStatement {
                statements: vec![Statement::Expression(Expression::Conditional(
                    Conditional {
                        condition: Box::new(Expression::Infix(InfixOperator {
                            token: Token::Equal,
                            left: Box::new(Expression::Identifier(Identifier {
                                token: Token::Ident("x".to_string()),
                                value: "x".to_string(),
                            })),
                            right: Box::new(Expression::Primitive(Primitive::IntegerLiteral(2))),
                        })),
                        consequence: BlockStatement {
                            statements: vec![Statement::Expression(Expression::ControlFlow(
                                ControlFlow::Break,
                            ))],
                        },
                        alternative: Some(BlockStatement {
                            statements: vec![Statement::Expression(Expression::ControlFlow(
                                ControlFlow::Continue,
                            ))],
                        }),
                    },
                ))],
            },
        };

        println!("Input:\n{input}");
        let program = generate_program(input);
        println!("Parsed:\n{program}");

        assert_eq!(program.statements.len(), 1);

        match program.statements[0].clone() {
            Statement::While(smt) => {
                assert_eq!(smt, expected);
            }
            _ => panic!("It is not an expression"),
        }
    }

    fn generate_program(input: &str) -> Program {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parse_errors(&parser);
        program
    }

    fn check_identifier(exp: &Identifier, value: &str) {
        assert_eq!(exp.value, value);
    }

    fn check_prefix_expression(exp: &Expression, operator: &str, right: &str) {
        match exp {
            Expression::Prefix(p) => {
                assert_eq!(p.token.to_string(), operator);
                assert_eq!(p.right.to_string(), right);
            }
            _ => panic!("It is not an prefix operator"),
        }
    }

    fn check_primitive_literal(exp: &Expression, value: &str) {
        match exp {
            Expression::Primitive(p) => match p {
                Primitive::IntegerLiteral(i) => assert_eq!(i.to_string(), value),
                Primitive::BooleanLiteral(b) => assert_eq!(b.to_string(), value),
                Primitive::StringLiteral(s) => assert_eq!(s, value),
            },
            _ => panic!("It is not a literal"),
        }
    }

    fn check_infix_expression(exp: &Expression, left: &str, operator: &str, right: &str) {
        match exp {
            Expression::Infix(p) => {
                check_primitive_literal(p.left.as_ref(), left);
                assert_eq!(operator, p.token.to_string());
                check_primitive_literal(p.right.as_ref(), right);
            }
            _ => panic!("It is not an infix expression"),
        }
    }

    fn check_conditional_expression(
        exp: &Expression,
        condition: &str,
        consequence: &str,
        alternative: Option<&str>,
    ) {
        match exp {
            Expression::Conditional(p) => {
                assert_eq!(format!("({condition})"), p.condition.as_ref().to_string());
                check_block_statement(&p.consequence, consequence);
                match alternative {
                    Some(a) => check_block_statement(p.alternative.as_ref().unwrap(), a),
                    None => assert!(p.alternative.is_none()),
                }
            }
            _ => panic!("It is not a conditional expression"),
        }
    }

    fn check_block_statement(statement: &BlockStatement, expected: &str) {
        if expected.is_empty() {
            assert_eq!(statement.to_string(), ""); // Empty block statement does not contain a
                                                   // newline
        } else {
            assert_eq!(statement.to_string(), format!("{expected}\n"));
        }
    }

    fn check_function_literal(exp: &Expression, params: Vec<&str>, body: &str) {
        match exp {
            Expression::FunctionLiteral(p) => {
                assert_eq!(p.parameters.len(), params.len());
                for (i, param) in params.iter().enumerate() {
                    check_identifier(&p.parameters[i], param);
                }
                check_block_statement(&p.body, body);
            }
            _ => panic!("It is not a function literal"),
        }
    }

    fn check_function_call(exp: &Expression, name: &str, arguments: Vec<&str>) {
        match exp {
            Expression::FunctionCall(p) => {
                assert_eq!(p.function.to_string(), name);
                assert_eq!(p.arguments.len(), arguments.len());
                for (i, arg) in arguments.iter().enumerate() {
                    assert_eq!(p.arguments[i].to_string(), arg.to_owned().to_string());
                }
            }
            _ => panic!("It is not a function call"),
        }
    }

    fn check_index_expression(exp: &Expression, left: &str, index: &str) {
        match exp {
            Expression::IndexExpression(p) => {
                assert_eq!(p.left.to_string(), left);
                assert_eq!(p.index.to_string(), index);
            }
            _ => panic!("It is not an index expression"),
        }
    }
}
