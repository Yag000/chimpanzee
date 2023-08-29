pub mod ast;
pub mod parser_errors;
mod parser_tests;

use crate::{
    lexer::{token::Token, Lexer},
    parser::ast::{
        Expression, Identifier, LetStatement, Precedence, Program, ReturnStatement, Statement,
    },
};

use self::{
    ast::{BlockStatement, LoopStatements, WhileStatement},
    parser_errors::ParserErrors,
};

pub struct Parser {
    lexer: Lexer,

    pub errors: ParserErrors,
    pub current_token: Token,
    pub peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            errors: ParserErrors::new(),
            current_token: Token::Illegal(String::new()),
            peek_token: Token::Illegal(String::new()),
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
            Token::While => self.parse_while_statement().map(Statement::While),
            Token::Break | Token::Continue => self
                .parse_control_flow_statement()
                .map(Statement::LoopStatements),
            _ => self.parse_expression_statement().map(Statement::Expression),
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        if !self.expect_peek(&Token::Ident(String::new())) {
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

        let mut value = match Expression::parse(self, Precedence::Lowest) {
            Ok(x) => x,
            Err(s) => {
                self.push_error(s);
                return None;
            }
        };

        if let Expression::FunctionLiteral(literal) = &mut value {
            literal.name = Some(name.token.to_string());
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

    fn parse_while_statement(&mut self) -> Option<WhileStatement> {
        self.next_token();

        let condition = match Expression::parse(self, Precedence::Lowest) {
            Ok(x) => x,
            Err(s) => {
                self.push_error(s);
                return None;
            }
        };

        if !self.expect_peek(&Token::LSquirly) {
            return None;
        }

        let body = BlockStatement::parse(self);

        Some(WhileStatement { condition, body })
    }

    fn parse_control_flow_statement(&mut self) -> Option<LoopStatements> {
        let ctrlflow = LoopStatements::parse(self).ok();
        self.next_token();
        ctrlflow
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
        self.errors.add_error(format!(
            "Expected next token to be {}, got {} instead",
            token, self.peek_token
        ));
    }

    pub fn peek_precedence(&mut self) -> Precedence {
        Precedence::from(&self.peek_token)
    }

    pub fn current_precedence(&mut self) -> Precedence {
        Precedence::from(&self.current_token)
    }

    fn push_error(&mut self, message: String) {
        if !message.is_empty() {
            self.errors.add_error(message);
        }
    }
}

pub fn parse(input: &str) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse_program()
}
