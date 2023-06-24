use crate::{
    evaluator::object::Object,
    parser::ast::{BlockStatement, Conditional, Expression, Primitive, Statement},
    Program, Token,
};

const TRUE: Object = Object::BOOLEAN(true);
const FALSE: Object = Object::BOOLEAN(false);
const NULL: Object = Object::NULL;

pub fn eval_program(program: &Program) -> Option<Object> {
    eval_statements(&program.statements)
}

fn eval_block_statemet(block: &BlockStatement) -> Option<Object> {
    eval_statements(&block.statements)
}

fn eval_statements(statements: &Vec<Statement>) -> Option<Object> {
    let mut result = None;
    for statement in statements.iter() {
        result = eval_statement(&statement);
    }
    result
}

fn eval_statement(statement: &Statement) -> Option<Object> {
    match statement {
        Statement::Expression(x) => eval_expression(x),
        _ => unimplemented!(),
    }
}

fn eval_expression(expression: &Expression) -> Option<Object> {
    match expression {
        Expression::Primitive(x) => eval_primitive_expression(x),
        Expression::Prefix(operator) => {
            let right = eval_expression(&operator.right).unwrap_or(NULL);
            eval_prefix_expression(&operator.token, &right)
        }
        Expression::Infix(operator) => {
            let left = eval_expression(&operator.left).unwrap_or(NULL);
            let right = eval_expression(&operator.right).unwrap_or(NULL);
            eval_infix_expression(&operator.token, &left, &right)
        }
        Expression::Conditional(conditional) => eval_conditional_expression(conditional),
        _ => unimplemented!(),
    }
}

fn eval_primitive_expression(expression: &Primitive) -> Option<Object> {
    match expression {
        Primitive::IntegerLiteral(x) => Some(Object::INTEGER(x.clone())),
        Primitive::BooleanLiteral(x) => Some(if *x { TRUE } else { FALSE }),
    }
}

fn eval_prefix_expression(operator: &Token, right: &Object) -> Option<Object> {
    match operator {
        Token::Bang => eval_bang_operator_expression(right),
        Token::Minus => eval_minus_operator_expression(right),
        _ => Some(NULL),
    }
}

fn eval_bang_operator_expression(right: &Object) -> Option<Object> {
    match right {
        Object::BOOLEAN(true) => Some(FALSE),
        Object::BOOLEAN(false) => Some(TRUE),
        Object::NULL => Some(TRUE),
        _ => Some(FALSE),
    }
}

fn eval_minus_operator_expression(right: &Object) -> Option<Object> {
    match right {
        Object::INTEGER(x) => Some(Object::INTEGER(-x)),
        _ => Some(NULL),
    }
}

fn eval_infix_expression(operator: &Token, left: &Object, right: &Object) -> Option<Object> {
    match (left, right) {
        (Object::INTEGER(x), Object::INTEGER(y)) => eval_integer_infix_expression(operator, x, y),
        (Object::BOOLEAN(x), Object::BOOLEAN(y)) => eval_boolean_infix_expression(operator, x, y),
        _ => Some(NULL),
    }
}

fn eval_integer_infix_expression(operator: &Token, left: &i64, right: &i64) -> Option<Object> {
    match operator {
        Token::Plus => Some(Object::INTEGER(left + right)),
        Token::Minus => Some(Object::INTEGER(left - right)),
        Token::Asterisk => Some(Object::INTEGER(left * right)),
        Token::Slash => Some(Object::INTEGER(left / right)),
        Token::LT => Some(Object::BOOLEAN(left < right)),
        Token::GT => Some(Object::BOOLEAN(left > right)),
        Token::Equal => Some(Object::BOOLEAN(left == right)),
        Token::NotEqual => Some(Object::BOOLEAN(left != right)),
        _ => Some(NULL),
    }
}

fn eval_boolean_infix_expression(operator: &Token, left: &bool, right: &bool) -> Option<Object> {
    match operator {
        Token::Equal => Some(Object::BOOLEAN(left == right)),
        Token::NotEqual => Some(Object::BOOLEAN(left != right)),
        _ => Some(NULL),
    }
}

fn eval_conditional_expression(conditional: &Conditional) -> Option<Object> {
    let condition = eval_expression(&conditional.condition).unwrap_or(NULL);
    if is_truthy(&condition) {
        eval_block_statemet(&conditional.consequence)
    } else if let Some(alternative) = &conditional.alternative {
        eval_block_statemet(alternative)
    } else {
        Some(NULL)
    }
}

fn is_truthy(object: &Object) -> bool {
    match object {
        Object::NULL => false,
        Object::BOOLEAN(x) => *x,
        _ => true,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Lexer, Parser};

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input.to_string());
            test_integer_object(evaluated, expected);
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = vec![
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input.to_string());
            test_boolean_object(evaluated, expected);
        }
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input.to_string());
            test_boolean_object(evaluated,expected);
        }
    }

    #[test]
    fn test_if_else_expression() {
        let tests = vec![
            ("if (true) { 10 }", Some(10)),
            ("if (false) { 10 }", None),
            ("if (1) { 10 }", Some(10)),
            ("if (1 < 2) { 10 }", Some(10)),
            ("if (1 > 2) { 10 }", None),
            ("if (1 > 2) { 10 } else { 20 }", Some(20)),
            ("if (1 < 2) { 10 } else { 20 }", Some(10)),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input.to_string());
            if let Some(expected) = expected {
                test_integer_object(evaluated, expected);
            } else {
                test_null_object(evaluated);
            }
        }
    }

    fn test_eval(input: String) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        eval_program(&program).unwrap()
    }

    fn test_integer_object(object: Object, expected: i64) {
        match object {
            Object::INTEGER(x) => assert_eq!(x, expected),
            _ => assert!(false, "The object is not an integer"),
        }
    }

    fn test_boolean_object(object: Object, expected: bool) {
        match object {
            Object::BOOLEAN(x) => assert_eq!(x, expected),
            _ => assert!(false, "The object is not a boolean"),
        }
    }

    fn test_null_object(object: Object) {
        match object {
            Object::NULL => (),
            _ => assert!(false, "The object is not null"),
        }
    }
}
