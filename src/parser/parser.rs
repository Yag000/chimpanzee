use crate::{Lexer, Token};

use super::ast::*;

pub struct Parser {
    lexer: Lexer,

    pub errors: Vec<String>,
    pub current_token: Token,
    pub peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            errors: Vec::new(),
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token != Token::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement().map(Statement::Let),
            Token::Return => self.parse_return_statement().map(Statement::Return),
            _ => self.parse_expression_statement().map(Statement::Expression),
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        if !self.expect_peek(&Token::Ident("".to_string())) {
            return None;
        }

        let name = match self.current_token.clone() {
            Token::Ident(value) => Identifier {
                token: self.current_token.clone(),
                value,
            },
            _ => unreachable!("This should never happen, we already checked for Ident"),
        };

        if !self.expect_peek(&Token::Assign) {
            return None;
        }

        self.next_token();

        let value = match Expression::parse(self, Precedence::Lowest) {
            Ok(x) => x,
            Err(s) => {
                self.push_error(s);
                return None;
            }
        };

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(LetStatement { name, value })
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        self.next_token();

        let return_value = match Expression::parse(self, Precedence::Lowest) {
            Ok(x) => x,
            Err(s) => {
                self.push_error(s);
                return None;
            }
        };

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(ReturnStatement { return_value })
    }

    fn parse_expression_statement(&mut self) -> Option<Expression> {
        let expression = Expression::parse(self, Precedence::Lowest);
        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        match expression {
            Ok(expression) => Some(expression),
            Err(s) => {
                self.push_error(s);
                None
            }
        }
    }

    pub fn current_token_is(&self, token: &Token) -> bool {
        // TODO: This is a hack, we need to implement PartialEq correctly for Token
        match self.current_token {
            Token::Ident(_) => matches!(token, Token::Ident(_)),
            Token::Int(_) => matches!(token, Token::Int(_)),
            _ => &self.current_token == token,
        }
    }

    pub fn peek_token_is(&self, token: &Token) -> bool {
        match self.peek_token {
            Token::Ident(_) => matches!(token, Token::Ident(_)),
            Token::Int(_) => matches!(token, Token::Int(_)),
            _ => &self.peek_token == token,
        }
    }

    pub fn expect_peek(&mut self, token: &Token) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            self.peek_error(token);
            false
        }
    }

    fn peek_error(&mut self, token: &Token) {
        self.errors.push(format!(
            "Expected next token to be {}, got {} instead",
            token, self.peek_token
        ));
    }

    pub fn peek_precedence(&mut self) -> Precedence {
        precedence_of(&self.peek_token)
    }

    pub fn current_precedence(&mut self) -> Precedence {
        precedence_of(&self.current_token)
    }

    fn push_error(&mut self, message: String) {
        if !message.is_empty() {
            self.errors.push(message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"let x = 5;
        let y = true;
        let foobar = y;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

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

        check_parse_errors(&parser);
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
        return 5;
        return true;
        return y;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

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

        check_parse_errors(&parser);
    }

    fn check_parse_errors(parser: &Parser) {
        let len = parser.errors.len();

        if len > 0 {
            println!("Parser has {} errors", parser.errors.len());
            for error in parser.errors.iter() {
                println!("Parser error: {}", error);
            }
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

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        parser.parse_program();

        assert_ne!(parser.errors.len(), 0);
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parse_errors(&parser);

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
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parse_errors(&parser);

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        assert_eq!(
            statement,
            &Statement::Expression(Expression::Primitive(Primitive::IntegerLiteral(5)))
        )
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
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parse_errors(&parser);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => check_prefix_expression(exp, operator, value),
                _ => assert!(false, "It is not an expression statement"),
            }
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
            ("5 < 5;", "5", "<", "5"),
            ("5 == 5;", "5", "==", "5"),
            ("5 != 5;", "5", "!=", "5"),
            ("true == true", "true", "==", "true"),
            ("true != false", "true", "!=", "false"),
            ("false == false", "false", "==", "false"),
        ];

        for (input, left, operator, right) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parse_errors(&parser);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => check_infix_expression(exp, left, operator, right),
                _ => assert!(false, "It is not an expression statement"),
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
        ];

        for (input, expected) in test {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parse_errors(&parser);
            print!("{}", program.to_string());
            assert_ne!(program.statements.len(), 0);
            assert_eq!(program.to_string(), format!("{expected}\n"));
        }
    }

    #[test]
    fn test_boolean_expression() {
        let tests = vec![("true;", true), ("false;", false)];

        for (input, expected) in tests {
            let lexer = Lexer::new(input.to_string());

            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parse_errors(&parser);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => check_primitive_literal(exp, &expected.to_string()),
                _ => assert!(false, "It is not an expression statement"),
            }
        }
    }

    #[test]
    fn test_if_statement() {
        let (input, condition, consequence, alternative) = ("if (x < y) { x }", "x < y", "x", None);
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parse_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => {
                check_conditional_expression(&exp, condition, consequence, alternative)
            }
            _ => assert!(false, "It is not an expression statement"),
        }
    }

    #[test]
    fn test_if_else_statement() {
        let (input, condition, consequence, alternative) =
            ("if (x < y) { x } else {y}", "x < y", "x", Some("y"));
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parse_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => {
                check_conditional_expression(&exp, condition, consequence, alternative)
            }
            _ => assert!(false, "It is not an expression statement"),
        }
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn(x, y) { x + y; }";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parse_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => check_function_literal(exp, vec!["x", "y"], "(x + y)"),
            _ => assert!(false, "It is not an expression statement"),
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
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parse_errors(&parser);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => check_function_literal(exp, expected, ""),
                _ => assert!(false, "It is not an expression statement"),
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

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parse_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(exp) => check_function_call(exp, name, argumnets),
            _ => assert!(false, "It is not an expression statement"),
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
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parse_errors(&parser);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => check_function_call(exp, name, argumnets),
                _ => assert!(false, "It is not an expression statement"),
            }
        }
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
            _ => assert!(false, "It is not an prefix operator"),
        }
    }

    fn check_primitive_literal(exp: &Expression, value: &str) {
        match exp {
            Expression::Primitive(p) => match p {
                Primitive::IntegerLiteral(i) => assert_eq!(i.to_string(), value),
                Primitive::BooleanLiteral(b) => assert_eq!(b.to_string(), value),
            },
            _ => assert!(false, "It is not a literal"),
        }
    }

    fn check_infix_expression(exp: &Expression, left: &str, operator: &str, right: &str) {
        match exp {
            Expression::Infix(p) => {
                check_primitive_literal(p.left.as_ref(), left);
                assert_eq!(operator, p.token.to_string());
                check_primitive_literal(p.right.as_ref(), right);
            }
            _ => assert!(false, "It is not an infix expression"),
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
                    Some(a) => check_block_statement(&p.alternative.as_ref().unwrap(), a),
                    None => assert!(p.alternative.is_none()),
                }
            }
            _ => assert!(false, "It is not a conditional expression"),
        }
    }

    fn check_block_statement(statement: &BlockStatement, expected: &str) {
        if expected == "" {
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
            _ => assert!(false, "It is not a function literal"),
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
            _ => assert!(false, "It is not a function call"),
        }
    }
}
