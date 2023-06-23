use std::error::Error;

use crate::{Lexer, Token};

struct Program {
    statements: Vec<Statement>,
}

#[derive(PartialEq, Debug)]
enum Expression {
    Temporary,
    IntegerLiteral(i64),
}

#[derive(PartialEq, Debug)]
enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

#[derive(PartialEq, Debug)]
struct LetStatement {
    name: Identifier,
    value: Expression,
}

#[derive(PartialEq, Debug)]
struct Identifier {
    token: Token,
    value: String,
}

#[derive(PartialEq, Debug)]
struct ReturnStatement {
    return_value: Expression,
}

struct Parser {
    lexer: Lexer,

    errors: Vec<String>,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(lexer: Lexer) -> Parser {
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

    fn next_token(&mut self) {
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
            _ => None, //TODO: Handle this error
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

    fn peek_token_is(&self, token: &Token) -> bool {
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

        test_errors(&parser);
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

        test_errors(&parser);
    }

    fn test_errors(parser: &Parser) {
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
    fn test_errors_number() {
        let input = r#"
        let x 5;
        let = 10;
        let 838383;
        let x = 838383;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        parser.parse_program();

        assert_eq!(parser.errors.len(), 3);
    }
}
