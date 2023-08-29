use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)] // I should find a way of avoiding this thanks to lifetimes, but
                                   // not for now (the issue is with the parser...)
pub enum Token {
    Illegal(String),
    Eof,

    // Identifiers + literals
    Ident(String), // add, foobar, x, y, ...
    Int(String),
    String(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    LTE,
    GTE,
    Equal,
    NotEqual,
    And,
    Or,

    // Delimiters
    Comma,
    Semicolon,

    LParen,   // (
    RParen,   // )
    LSquirly, // {
    RSquirly, // }
    LSquare,  // [
    RSquare,  // ]
    Colon,    // :

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    While,
    Break,
    Continue,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(x) | Token::Int(x) | Token::String(x) => write!(f, "{x}"),
            Token::Illegal(s) => write!(f, "Illegal: {s}"),
            Token::Eof => write!(f, "Eof"),
            Token::Assign => write!(f, "="),
            Token::Bang => write!(f, "!"),
            Token::Minus => write!(f, "-"),
            Token::Slash => write!(f, "/"),
            Token::Asterisk => write!(f, "*"),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::LT => write!(f, "<"),
            Token::GT => write!(f, ">"),
            Token::LTE => write!(f, "<="),
            Token::GTE => write!(f, ">="),
            Token::Plus => write!(f, "+"),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LSquirly => write!(f, "{{"),
            Token::RSquirly => write!(f, "}}"),
            Token::LSquare => write!(f, "["),
            Token::RSquare => write!(f, "]"),
            Token::Colon => write!(f, ":"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::While => write!(f, "while"),
            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
        }
    }
}
