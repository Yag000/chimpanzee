use crate::{Lexer, Token};

use super::ast::*;

pub struct Parser {
    lexer: Lexer,

    errors: Vec<String>,
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

    fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token != Token::Eof {
            let statement = self.parse_statement();
            match statement {
                Some(statement) => program.statements.push(statement),
                None => (),
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
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
        // TODO: Parse the expression, for now we just skip until semicolon and set the field to
        // Temporary
        while !self.current_token_is(&Token::Semicolon) {
            self.next_token();
        }

        return Some(LetStatement {
            name,
            value: Expression::Temporary,
        });
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        self.next_token();

        //TODO: Parse the expression, for now we just skip until semicolon and set the field to
        //Temporary
        while !self.current_token_is(&Token::Semicolon) {
            self.next_token();
        }

        return Some(ReturnStatement {
            return_value: Expression::Temporary,
        });
    }

    fn parse_expression_statement(&mut self) -> Option<Expression> {
        let expression = Expression::parse(self, Precedence::Lowest);
        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        match expression {
            Ok(expression) => Some(expression),
            Err(s) => {
                self.errors.push(s);
                None
            }
        }
    }

    fn current_token_is(&self, token: &Token) -> bool {
        // TODO: This is a hack, we need to implement PartialEq correctly for Token
        match self.current_token {
            Token::Ident(_) => match token {
                Token::Ident(_) => true,
                _ => false,
            },
            Token::Int(_) => match token {
                Token::Int(_) => true,
                _ => false,
            },
            _ => &self.current_token == token,
        }
    }

    pub fn peek_token_is(&self, token: &Token) -> bool {
        match self.peek_token {
            Token::Ident(_) => match token {
                Token::Ident(_) => true,
                _ => false,
            },
            Token::Int(_) => match token {
                Token::Int(_) => true,
                _ => false,
            },
            _ => &self.peek_token == token,
        }
    }

    fn expect_peek(&mut self, token: &Token) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            return true;
        } else {
            self.peek_error(token);
            return false;
        }
    }

    fn peek_error(&mut self, token: &Token) {
        self.errors.push(format!(
            "Expected next token to be {:?}, got {:?} instead",
            token, self.peek_token
        ));
    }

    pub fn peek_precedence(&mut self) -> Precedence {
        precedence_of(&self.peek_token)
    }

    pub fn current_precedence(&mut self) -> Precedence {
        precedence_of(&self.current_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"let x = 5;
        let y = 10;
        let foobar = 838383;
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
                value: Expression::Temporary,
            }),
            Statement::Let(LetStatement {
                name: Identifier {
                    token: Token::Ident("y".to_string()),
                    value: "y".to_string(),
                },
                value: Expression::Temporary,
            }),
            Statement::Let(LetStatement {
                name: Identifier {
                    token: Token::Ident("foobar".to_string()),
                    value: "foobar".to_string(),
                },
                value: Expression::Temporary,
            }),
        ];

        assert_eq!(program.statements.len(), 3);

        for (i, expected) in expected_statemets.iter().enumerate() {
            assert_eq!(program.statements[i], *expected);
        }

        check_parse_errors(&parser);
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
        return 5;
        return 10;
        return 993322;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let expected = vec![
            Statement::Return(ReturnStatement {
                return_value: Expression::Temporary,
            }),
            Statement::Return(ReturnStatement {
                return_value: Expression::Temporary,
            }),
            Statement::Return(ReturnStatement {
                return_value: Expression::Temporary,
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
    fn test_string_conversion() {
        let program = Program {
            statements: vec![
                Statement::Let(LetStatement {
                    name: Identifier {
                        token: Token::Ident("myVar".to_string()),
                        value: "myVar".to_string(),
                    },
                    value: Expression::Identifier(Identifier {
                        token: Token::Ident("anotherVar".to_string()),
                        value: "anotherVar".to_string(),
                    }),
                }),
                Statement::Return(ReturnStatement {
                    return_value: Expression::Identifier(Identifier {
                        token: Token::Ident("myVar".to_string()),
                        value: "myVar".to_string(),
                    }),
                }),
            ],
        };

        assert_eq!(
            program.to_string(),
            "let myVar = anotherVar;\nreturn myVar;\n"
        );
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
            &Statement::Expression(Expression::IntegerLiteral(5))
        )
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let tests = vec![("!5", "!", 5), ("-15", "-", 15)];

        for (input, operator, value) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parse_errors(&parser);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => match exp {
                    Expression::Prefix(p) => {
                        assert_eq!(operator, p.token.to_string());
                        assert_eq!(p.right.as_ref(), &Expression::IntegerLiteral(value));
                    }
                    _ => assert!(false, "It is not an prefix operator"),
                },
                _ => assert!(false, "It is not an expression statement"),
            }
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let tests = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for (input, left, operator, right) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parse_errors(&parser);

            assert_eq!(program.statements.len(), 1);
            match &program.statements[0] {
                Statement::Expression(exp) => match exp {
                    Expression::Infix(p) => {
                        assert_eq!(p.left.as_ref(), &Expression::IntegerLiteral(left));
                        assert_eq!(operator, p.token.to_string());
                        assert_eq!(p.right.as_ref(), &Expression::IntegerLiteral(right));
                    }
                    _ => assert!(false, "It is not an prefix operator"),
                },
                _ => assert!(false, "It is not an expression statement"),
            }
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let test = vec![
            ("-a * b", "((-a) * b)\n"),
            ("!-a", "(!(-a))\n"),
            ("a + b + c", "((a + b) + c)\n"),
            ("a + b - c", "((a + b) - c)\n"),
            ("a * b * c", "((a * b) * c)\n"),
            ("a * b / c", "((a * b) / c)\n"),
            ("a + b / c", "(a + (b / c))\n"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)\n"),
            ("3 + 4; -5 * 5", "(3 + 4)\n((-5) * 5)\n"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))\n"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))\n"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))\n",
            ),
        ];

        for (input, expected) in test {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parse_errors(&parser);
            print!("{}", program.to_string());
            assert_ne!(program.statements.len(), 0);
            assert_eq!(program.to_string(), expected);
        }
    }
}