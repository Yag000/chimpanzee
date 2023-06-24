use crate::Token;
use std::fmt::Display;

use super::parser::Parser;

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut program = String::new();
        for statement in &self.statements {
            program.push_str(&format!("{}\n", statement));
        }
        write!(f, "{}", program)
    }
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Temporary, // TODO: remove this
    Identifier(Identifier),
    Primitive(Primitive),
    Prefix(PrefixOperator),
    Infix(InfixOperator),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Temporary => write!(f, "Temporary"),
            Expression::Identifier(x) => write!(f, "{}", x),
            Expression::Primitive(x) => write!(f, "{}", x),
            Expression::Prefix(x) => write!(f, "{}", x),
            Expression::Infix(x) => write!(f, "{}", x),
        }
    }
}

impl Expression {
    pub fn parse(parser: &mut Parser, precedence: Precedence) -> Result<Self, String> {
        let mut left_exp = match parser.current_token.clone() {
            Token::Ident(_) => (Identifier::parse(parser)).map(Expression::Identifier),
            Token::Int(_) | Token::False | Token::True => {
                Primitive::parse(parser).map(Expression::Primitive)
            }
            Token::Bang | Token::Minus => PrefixOperator::parse(parser).map(Expression::Prefix),
            _ => Err(format!(
                "There is no prefix parser for the token {:?}",
                parser.current_token
            )),
        }?;

        while !parser.peek_token_is(&Token::Semicolon) && precedence < parser.peek_precedence() {
            match &parser.peek_token {
                Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Asterisk
                | Token::Equal
                | Token::NotEqual
                | Token::LT
                | Token::GT => {
                    parser.next_token(); // TODO: Solve this.
                                         //  This is absolutely awful, I need to peek the next token
                                         //  only if a infix operator is found, I want to also
                                         //  avoid a double match
                    left_exp = Expression::Infix(match InfixOperator::parse(parser, left_exp) {
                        Ok(x) => x,
                        Err(x) => return Err(x),
                    });
                }

                _ => return Ok(left_exp),
            }
        }

        return Ok(left_exp);
    }
}

#[derive(PartialEq, Debug)]
pub enum Primitive {
    IntegerLiteral(i64),
    BooleanLiteral(bool),
}

impl Primitive {
    fn parse(parser: &mut Parser) -> Result<Self, String> {
        match parser.current_token.clone() {
            Token::Int(x) => match x.parse::<i64>() {
                Ok(x) => Ok(Primitive::IntegerLiteral(x)),
                Err(_) => Err("Error: expected a number, found an incopatible string".to_string()),
            },
            Token::True => Ok(Primitive::BooleanLiteral(true)),
            Token::False => Ok(Primitive::BooleanLiteral(false)),
            _ => Err(format!(
                "There is no primitive parser for the token {:?}",
                parser.current_token
            )),
        }
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::IntegerLiteral(x) => write!(f, "{}", x),
            Primitive::BooleanLiteral(x) => write!(f, "{}", x),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct PrefixOperator {
    pub token: Token,
    pub right: Box<Expression>,
}

impl PrefixOperator {
    pub fn new(token: Token, rigth: Expression) -> Self {
        PrefixOperator {
            token,
            right: Box::new(rigth),
        }
    }
    fn parse(parser: &mut Parser) -> Result<Self, String> {
        let token = parser.current_token.clone();
        parser.next_token();
        let right = Expression::parse(parser, Precedence::Prefix)?;
        Ok(PrefixOperator::new(token, right))
    }
}
impl Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.token, self.right)
    }
}

#[derive(PartialEq, Debug)]
pub struct InfixOperator {
    pub token: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl InfixOperator {
    pub fn new(token: Token, left: Expression, right: Expression) -> Self {
        InfixOperator {
            token,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn parse(parser: &mut Parser, left: Expression) -> Result<Self, String> {
        let token = parser.current_token.clone();
        let precedence = parser.current_precedence();
        parser.next_token();
        let right = Expression::parse(parser, precedence)?;
        Ok(InfixOperator::new(token, left, right))
    }
}

impl Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.token, self.right)
    }
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(statement) => write!(f, "{}", statement),
            Statement::Return(statement) => write!(f, "{}", statement),
            Statement::Expression(expression) => write!(f, "{}", expression),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {} = {};", self.name, self.value)
    }
}

#[derive(PartialEq, Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl Identifier {
    fn parse(parser: &mut Parser) -> Result<Self, String> {
        return Ok(Identifier{token:parser.current_token.clone(), value:match parser.current_token.clone(){ // TODO: Improve this with lifetimes
           Token::Ident(s) => s,
           _=> panic!("This should be a Token::Ident, if not the function has not been properly called"),
        }});
    }
}

#[derive(PartialEq, Debug)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {};", &self.return_value)
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 0,
    Equals = 1,      // ==
    LessGreater = 2, // > or <
    Sum = 3,         // +
    Product = 4,     // *
    Prefix = 5,      // -X or !X
    Call = 6,        // myFunction(X)
}

pub fn precedence_of(token: &Token) -> Precedence {
    match token {
        Token::Equal | Token::NotEqual => Precedence::Equals,
        Token::LT | Token::GT => Precedence::LessGreater,
        Token::Plus | Token::Minus => Precedence::Sum,
        Token::Slash | Token::Asterisk => Precedence::Product,
        _ => Precedence::Lowest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
