pub mod token;

use std::iter::FromIterator;

use crate::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char,             // current char under examination
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::LTE
                } else {
                    Token::LT
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::GTE
                } else {
                    Token::GT
                }
            }
            '&' => {
                if self.peek_char() == '&' {
                    self.read_char();
                    Token::And
                } else {
                    Token::Illegal
                }
            }
            '|' => {
                if self.peek_char() == '|' {
                    self.read_char();
                    Token::Or
                } else {
                    Token::Illegal
                }
            }
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '{' => Token::LSquirly,
            '}' => Token::RSquirly,
            '[' => Token::LSquare,
            ']' => Token::RSquare,
            ':' => Token::Colon,
            '"' => {
                let string = self.read_string();
                Token::String(string)
            }
            '\0' => Token::Eof,
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident_string = self.read_identifier();
                return match ident_string.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Ident(ident_string),
                };
            }
            '0'..='9' => return Token::Int(self.read_number()),
            _ => Token::Illegal,
        };
        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        String::from_iter(self.input[position..self.position].iter())
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        String::from_iter(self.input[position..self.position].iter())
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break; // TODO: handle unterminated string
            }
        }
        String::from_iter(self.input[position..self.position].iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token_basic() {
        let input = "=+(){},;";

        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LSquirly,
            Token::RSquirly,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in expected {
            let token = lexer.next_token();

            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_next_token_complete() {
        let input = r#"let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);

            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;

            "foobar"
            "foo bar"
            [1, 2];
            {"foo": "bar"}
            true && false || true && false;
            12 <= 12 && 12 >= 12;
        "#;

        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LSquirly,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RSquirly,
            Token::Semicolon,
            //
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Int(String::from("5")),
            Token::LT,
            Token::Int(String::from("10")),
            Token::GT,
            Token::Int(String::from("5")),
            Token::Semicolon,
            //
            Token::If,
            Token::LParen,
            Token::Int(String::from("5")),
            Token::LT,
            Token::Int(String::from("10")),
            Token::RParen,
            Token::LSquirly,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RSquirly,
            Token::Else,
            Token::LSquirly,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RSquirly,
            //
            Token::Int(String::from("10")),
            Token::Equal,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Int(String::from("10")),
            Token::NotEqual,
            Token::Int(String::from("9")),
            Token::Semicolon,
            //
            Token::String(String::from("foobar")),
            Token::String(String::from("foo bar")),
            //
            Token::LSquare,
            Token::Int(String::from("1")),
            Token::Comma,
            Token::Int(String::from("2")),
            Token::RSquare,
            Token::Semicolon,
            //
            Token::LSquirly,
            Token::String(String::from("foo")),
            Token::Colon,
            Token::String(String::from("bar")),
            Token::RSquirly,
            //
            Token::True,
            Token::And,
            Token::False,
            Token::Or,
            Token::True,
            Token::And,
            Token::False,
            Token::Semicolon,
            //
            Token::Int(String::from("12")),
            Token::LTE,
            Token::Int(String::from("12")),
            Token::And,
            Token::Int(String::from("12")),
            Token::GTE,
            Token::Int(String::from("12")),
            Token::Semicolon,
            //
            Token::Eof,
        ];

        for expected_token in expected {
            let token = lexer.next_token();
            println!("{:?}", token);
            assert_eq!(token, expected_token);
        }
    }
}
