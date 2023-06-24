use crate::{
    evaluator::object::Object,
    parser::ast::{Expression, Statement, Primitive},
    Lexer, Parser, Program,
};

pub fn eval_program(program: Program) -> Option<Object> {
    let mut result = None;
    for statement in program.statements {
        result = eval_statement(statement);
    }
    result
}

fn eval_statement(statement: Statement) -> Option<Object> {
    match statement {
        Statement::Expression(x) => eval_expression(x),
        _ => unimplemented!(),
    }
}

fn eval_expression(expression: Expression) -> Option<Object> {
    match expression{
        Expression::Primitive(x) => eval_primitive_expression(x),
        _ => unimplemented!(),

    }
}

fn eval_primitive_expression(expression: Primitive) -> Option<Object> {
    match expression {
        Primitive::IntegerLiteral(x) => Some(Object::INTEGER(x)),
        Primitive::BooleanLiteral(x) => Some(Object::BOOLEAN(x)),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![("5", 5), ("10", 10)];

        for (input, expected) in tests {
            let evaluated = test_eval(input.to_string());
            test_integer_object(evaluated, expected);
        }
    }

    fn test_eval(input: String) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        match eval_program(program) {
            Some(x) => x,
            None => panic!("The program did not produce an output"),
        }
    }

    fn test_integer_object(object: Object, expected: i64) {
        match object {
            Object::INTEGER(x) => assert_eq!(x, expected),
            _ => assert!(false, "The object is not an integer"),
        }
    }
}
