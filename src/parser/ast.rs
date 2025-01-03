use enum_stringify::EnumStringify;

use crate::{lexer::token::Token, parser::Parser};
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut program = String::new();
        for statement in &self.statements {
            program.push_str(&format!("{statement}\n"));
        }
        write!(f, "{program}")
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Primitive(Primitive),
    Prefix(PrefixOperator),
    Infix(InfixOperator),
    Conditional(Conditional),
    FunctionLiteral(FunctionLiteral),
    FunctionCall(FunctionCall),
    ArrayLiteral(ArrayLiteral),
    HashMapLiteral(HashMapLiteral),
    IndexExpression(IndexExpression),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(x) => write!(f, "{x}"),
            Expression::Primitive(x) => write!(f, "{x}"),
            Expression::Prefix(x) => write!(f, "{x}"),
            Expression::Infix(x) => write!(f, "{x}"),
            Expression::Conditional(x) => write!(f, "{x}"),
            Expression::FunctionLiteral(x) => write!(f, "{x}"),
            Expression::FunctionCall(x) => write!(f, "{x}"),
            Expression::ArrayLiteral(x) => write!(f, "{x}"),
            Expression::IndexExpression(x) => write!(f, "{x}"),
            Expression::HashMapLiteral(x) => write!(f, "{x}"),
        }
    }
}

impl Expression {
    pub fn parse(parser: &mut Parser, precedence: Precedence) -> Result<Self, String> {
        let mut left_exp = match parser.current_token.clone() {
            Token::Ident(_) => (Identifier::parse(parser)).map(Expression::Identifier),
            Token::Int(_) | Token::False | Token::True | Token::String(_) => {
                Primitive::parse(parser).map(Expression::Primitive)
            }
            Token::Bang | Token::Minus => PrefixOperator::parse(parser).map(Expression::Prefix),
            Token::LParen => Self::parse_grouped_expression(parser),
            Token::If => Conditional::parse(parser).map(Expression::Conditional),
            Token::Function => FunctionLiteral::parse(parser).map(Expression::FunctionLiteral),
            Token::LSquare => ArrayLiteral::parse(parser).map(Expression::ArrayLiteral),
            Token::LSquirly => HashMapLiteral::parse(parser).map(Expression::HashMapLiteral),

            _ => Err(format!(
                "There is no prefix parser for the token {}",
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
                | Token::GT
                | Token::LTE
                | Token::GTE
                | Token::And
                | Token::Or
                | Token::Modulo => {
                    parser.next_token(); // TODO: Solve this.
                                         //  This is absolutely awful, I need to peek the next token
                                         //  only if a infix operator is found, I want to also
                                         //  avoid a double match
                    left_exp = Expression::Infix(InfixOperator::parse(parser, left_exp)?);
                }
                Token::LParen => {
                    parser.next_token();
                    left_exp = Expression::FunctionCall(FunctionCall::parse(parser, left_exp)?);
                }
                Token::LSquare => {
                    parser.next_token();
                    left_exp =
                        Expression::IndexExpression(IndexExpression::parse(parser, left_exp)?);
                }
                _ => return Ok(left_exp),
            }
        }

        Ok(left_exp)
    }

    fn parse_grouped_expression(parser: &mut Parser) -> Result<Expression, String> {
        parser.next_token();
        let exp = Expression::parse(parser, Precedence::Lowest);
        if parser.expect_peek(&Token::RParen) {
            exp
        } else {
            Err(String::new())
        }
    }

    fn parse_expression_list(parser: &mut Parser, end: &Token) -> Result<Vec<Expression>, String> {
        let mut list = Vec::new();
        if parser.peek_token_is(end) {
            parser.next_token();
            return Ok(list);
        }

        parser.next_token();
        list.push(Expression::parse(parser, Precedence::Lowest)?);
        while parser.peek_token_is(&Token::Comma) {
            parser.next_token();
            parser.next_token();
            list.push(Expression::parse(parser, Precedence::Lowest)?);
        }
        if !parser.expect_peek(end) {
            return Err(String::new());
        }
        Ok(list)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Primitive {
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    StringLiteral(String),
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
            Token::String(x) => Ok(Primitive::StringLiteral(x)),
            _ => Err(format!(
                "There is no primitive parser for the token {}",
                parser.current_token
            )),
        }
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::IntegerLiteral(x) => write!(f, "{x}"),
            Primitive::BooleanLiteral(x) => write!(f, "{x}"),
            Primitive::StringLiteral(x) => write!(f, "\"{x}\""),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Display for Conditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut exp = String::new();
        exp.push_str(&format!(
            "if {}{{\n{}}}",
            self.condition.as_ref(),
            self.consequence
        ));
        if let Some(alternative) = &self.alternative {
            exp.push_str(&format!(" else {{\n{alternative}}}"));
        }
        write!(f, "{exp}")
    }
}

impl Conditional {
    fn parse(parser: &mut Parser) -> Result<Self, String> {
        if !parser.expect_peek(&Token::LParen) {
            return Err(String::new());
        }
        parser.next_token();
        let condition = Expression::parse(parser, Precedence::Lowest)?;
        if !parser.expect_peek(&Token::RParen) {
            return Err(String::new());
        }
        if !parser.expect_peek(&Token::LSquirly) {
            return Err(String::new());
        }
        let consequence = BlockStatement::parse(parser);
        let mut alternative = None;

        if parser.peek_token_is(&Token::Else) {
            parser.next_token();
            if !parser.expect_peek(&Token::LSquirly) {
                return Err(String::new());
            }

            alternative = Some(BlockStatement::parse(parser));
        }

        Ok(Conditional {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut statements = String::new();
        for statement in &self.statements {
            statements.push_str(&format!("{statement}\n"));
        }
        write!(f, "{statements}")
    }
}

impl BlockStatement {
    pub(crate) fn parse(parser: &mut Parser) -> Self {
        parser.next_token();
        let mut statements: Vec<Statement> = Vec::new();
        while !parser.current_token_is(&Token::RSquirly) && !parser.current_token_is(&Token::Eof) {
            if let Some(x) = parser.parse_statement() {
                statements.push(x);
            }
            parser.next_token();
        }
        BlockStatement { statements }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionLiteral {
    pub name: Option<String>,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parameters = self
            .parameters
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        write!(f, "fn({}){{\n{}}}", parameters.join(", "), self.body)
    }
}

impl FunctionLiteral {
    fn parse(parser: &mut Parser) -> Result<Self, String> {
        if !parser.expect_peek(&Token::LParen) {
            return Err(String::new());
        }
        let parameters = Self::parse_function_parameters(parser)?;
        if !parser.expect_peek(&Token::LSquirly) {
            return Err(String::new());
        }
        let body = BlockStatement::parse(parser);
        Ok(FunctionLiteral {
            name: None,
            parameters,
            body,
        })
    }

    fn parse_function_parameters(parser: &mut Parser) -> Result<Vec<Identifier>, String> {
        let mut identifiers: Vec<Identifier> = Vec::new();

        if parser.peek_token_is(&Token::RParen) {
            parser.next_token();
            return Ok(identifiers);
        }

        parser.next_token();

        let mut identifier = Identifier::new(parser.current_token.clone());
        identifiers.push(identifier);

        while parser.peek_token_is(&Token::Comma) {
            parser.next_token();
            parser.next_token();
            identifier = Identifier::new(parser.current_token.clone());
            identifiers.push(identifier);
        }

        if !parser.expect_peek(&Token::RParen) {
            return Err(String::new());
        }

        Ok(identifiers)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCall {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arguments = self
            .arguments
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        write!(f, "{}({})", self.function, arguments.join(", "))
    }
}

impl FunctionCall {
    fn parse(parser: &mut Parser, function: Expression) -> Result<Self, String> {
        let arguments = Expression::parse_expression_list(parser, &Token::RParen)?;

        Ok(FunctionCall {
            function: Box::new(function),
            arguments,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(Expression),
    While(WhileStatement),
    LoopStatements(LoopStatement),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(statement) => write!(f, "{statement}"),
            Statement::Return(statement) => write!(f, "{statement}"),
            Statement::Expression(expression) => write!(f, "{expression}"),
            Statement::While(statement) => write!(f, "{statement}"),
            Statement::LoopStatements(statement) => write!(f, "{statement}"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {} = {};", self.name, self.value)
    }
}

#[derive(PartialEq, Debug, Clone)]
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
    fn new(token: Token) -> Self {
        match token.clone() {
            Token::Ident(s) => Identifier { token, value: s },
            _ => panic!(
                "This should be a Token::Ident; if not, the function has not been properly called."
            ),
        }
    }

    fn parse(parser: &mut Parser) -> Result<Self, String> {
        match parser.current_token.clone() {
            Token::Ident(s) => Ok(Identifier {
                token: parser.current_token.clone(),
                value: s,
            }),
            _ => Err(format!(
                "Expected an identifier, got {}",
                parser.current_token
            )),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {};", &self.return_value)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: BlockStatement,
}

impl Display for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {} {{\n{}}}", self.condition, self.body)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
}

impl Display for ArrayLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements = self
            .elements
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        write!(f, "[{}]", elements.join(", "))
    }
}

impl ArrayLiteral {
    fn parse(parser: &mut Parser) -> Result<Self, String> {
        let expresssions = Expression::parse_expression_list(parser, &Token::RSquare)?;
        Ok(ArrayLiteral {
            elements: expresssions,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct IndexExpression {
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

impl Display for IndexExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}[{}])", self.left, self.index)
    }
}

impl IndexExpression {
    fn parse(parser: &mut Parser, left: Expression) -> Result<Self, String> {
        parser.next_token();
        let index = Expression::parse(parser, Precedence::Lowest)?;
        if !parser.expect_peek(&Token::RSquare) {
            return Err(String::new());
        }
        Ok(IndexExpression {
            left: Box::new(left),
            index: Box::new(index),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashMapLiteral {
    pub pairs: Vec<(Expression, Expression)>,
}

impl Display for HashMapLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pairs = self
            .pairs
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<String>>();
        write!(f, "{{{}}}", pairs.join(", "))
    }
}

impl HashMapLiteral {
    fn parse(parser: &mut Parser) -> Result<Self, String> {
        let mut pairs = Vec::new();
        while !parser.peek_token_is(&Token::RSquirly) {
            parser.next_token();
            let key = Expression::parse(parser, Precedence::Lowest)?;
            if !parser.expect_peek(&Token::Colon) {
                return Err(String::new());
            }

            parser.next_token();
            let value = Expression::parse(parser, Precedence::Lowest)?;

            pairs.push((key, value));

            if !parser.peek_token_is(&Token::RSquirly) && !parser.expect_peek(&Token::Comma) {
                return Err(String::new());
            }
        }

        if !parser.expect_peek(&Token::RSquirly) {
            return Err(String::new());
        }

        Ok(HashMapLiteral { pairs })
    }
}

#[derive(PartialEq, Debug, Clone, EnumStringify)]
#[enum_stringify(case = "lower")]
pub enum LoopStatement {
    Break,
    Continue,
}

impl LoopStatement {
    pub fn parse(parser: &mut Parser) -> Result<Self, String> {
        match parser.current_token {
            Token::Break => Ok(Self::Break),
            Token::Continue => Ok(Self::Continue),
            _ => Err(format!(
                "Expected a loop statement keyword (break, continue), got {}",
                parser.current_token
            )),
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedence {
    Lowest = 0,
    Equals = 1,      // ==
    LessGreater = 2, // > or <
    Sum = 3,         // +
    Product = 4,     // *
    Prefix = 5,      // -X or !X
    Call = 6,        // myFunction(X)
    Index = 7,       // array[index]
}

impl From<&Token> for Precedence {
    fn from(value: &Token) -> Self {
        match value {
            Token::Equal | Token::NotEqual => Precedence::Equals,
            Token::LT | Token::GT | Token::LTE | Token::GTE => Precedence::LessGreater,
            Token::Plus | Token::Minus | Token::Or => Precedence::Sum,
            Token::Slash | Token::Asterisk | Token::And | Token::Modulo => Precedence::Product,
            Token::LParen => Precedence::Call,
            Token::LSquare => Precedence::Index,
            _ => Precedence::Lowest,
        }
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
