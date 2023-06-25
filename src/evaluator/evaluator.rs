use crate::{
    evaluator::object::Object,
    parser::ast::{BlockStatement, Conditional, Expression, Identifier, Primitive, Statement},
    Program, Token,
};

use super::{enviroment::Environment, object::FunctionObject};

const TRUE: Object = Object::BOOLEAN(true);
const FALSE: Object = Object::BOOLEAN(false);
const NULL: Object = Object::NULL;

pub fn eval_program(program: &Program, env: &mut Environment) -> Object {
    let mut result = NULL;
    for statement in program.statements.iter() {
        result = eval_statement(&statement, env);
        match result {
            Object::RETURN(x) => return *x,
            Object::ERROR(x) => return Object::ERROR(x),
            _ => (),
        }
    }
    result
}

fn eval_block_statemet(block: &BlockStatement, env: &mut Environment) -> Object {
    let mut result = NULL;
    for statement in block.statements.iter() {
        result = eval_statement(&statement, env);
        match result {
            Object::RETURN(_) | Object::ERROR(_) => return result,
            _ => (),
        }
    }
    result
}

fn eval_statement(statement: &Statement, env: &mut Environment) -> Object {
    match statement {
        Statement::Expression(x) => eval_expression(x, env),
        Statement::Return(x) => {
            let value = eval_expression(&x.return_value, env);
            if is_error(&value) {
                return value;
            }
            Object::RETURN(Box::new(value))
        }
        Statement::Let(x) => {
            let value = eval_expression(&x.value, env);
            if is_error(&value) {
                return value;
            }
            env.set(x.name.to_string(), value.clone()); // FIXME: this is a problem, we need to use references
            value
            }
    }
}

fn eval_expression(expression: &Expression, env: &mut Environment) ->  Object {
    match expression {
        Expression::Primitive(x) => eval_primitive_expression(x),
        Expression::Prefix(operator) => {
            let right = eval_expression(&operator.right, env);
            if is_error(&right) {
                return right;
            }
            eval_prefix_expression(&operator.token, &right)
        }
        Expression::Infix(operator) => {
            let left = eval_expression(&operator.left, env);
            if is_error(&left) {
                return left;
            }
            let right = eval_expression(&operator.right, env);
            if is_error(&right) {
                return right;
            }
            eval_infix_expression(&operator.token, &left, &right)
        }
        Expression::Conditional(conditional) => eval_conditional_expression(conditional, env),
        Expression::Identifier(x) => eval_identifier(&x, env),
        Expression::FunctionLiteral(x) => {
            let parameters = &x.parameters;
            let body = &x.body;
            return Object::FUNCTION(FunctionObject {
                parameters: parameters.clone(),
                body: body.clone(),
                environment: env.clone(), // TODO: this is a problem, we need to use references
            });
        }
        Expression::FunctionCall(x) => {
            let function = eval_expression(&x.function, env);
            if is_error(&function) {}
            let args = eval_expressions(&x.arguments, env);
            if args.len() == 1 && is_error(&args[0]) {
                return args[0].clone();
            }
            return apply_function(&function, &args);
        }
    }
}

fn eval_primitive_expression(expression: &Primitive) ->  Object {
    match expression {
        Primitive::IntegerLiteral(x) => Object::INTEGER(x.clone()),
        Primitive::BooleanLiteral(x) => if *x { TRUE } else { FALSE },
    }
}

fn eval_prefix_expression(operator: &Token, right: &Object) ->  Object {
    match operator {
        Token::Bang => eval_bang_operator_expression(right),
        Token::Minus => eval_minus_operator_expression(right),
        _ => Object::ERROR(format!(
            "unknown operator: {}{}",
            operator, right
        )),
    }
}

fn eval_bang_operator_expression(right: &Object) ->  Object {
    match right {
        Object::BOOLEAN(true) => FALSE,
        Object::BOOLEAN(false) => TRUE,
        Object::NULL => TRUE,
        _ => FALSE,
    }
}

fn eval_minus_operator_expression(right: &Object) ->  Object {
    match right {
        Object::INTEGER(x) => Object::INTEGER(-x),
        _ => Object::ERROR(format!("unknown operator: -{}", right)),
    }
}

fn eval_infix_expression(operator: &Token, left: &Object, right: &Object) ->  Object {
    match (left, right) {
        (Object::INTEGER(x), Object::INTEGER(y)) => eval_integer_infix_expression(operator, x, y),
        (Object::BOOLEAN(x), Object::BOOLEAN(y)) => eval_boolean_infix_expression(operator, x, y),
        _ => Object::ERROR(format!(
            "type mismatch: {} {} {}",
            left.get_type(),
            operator,
            right.get_type()
        )),
    }
}

fn eval_integer_infix_expression(operator: &Token, left: &i64, right: &i64) ->  Object {
    match operator {
        Token::Plus => Object::INTEGER(left + right),
        Token::Minus => Object::INTEGER(left - right),
        Token::Asterisk => Object::INTEGER(left * right),
        Token::Slash => Object::INTEGER(left / right),
        Token::LT => Object::BOOLEAN(left < right),
        Token::GT => Object::BOOLEAN(left > right),
        Token::Equal => Object::BOOLEAN(left == right),
        Token::NotEqual => Object::BOOLEAN(left != right),
        _ => Object::ERROR(format!(
            "unknown operator: INTEGER {} INTEGER",
            operator
        )),
    }
}

fn eval_boolean_infix_expression(operator: &Token, left: &bool, right: &bool) ->  Object {
    match operator {
        Token::Equal => Object::BOOLEAN(left == right),
        Token::NotEqual => Object::BOOLEAN(left != right),
        _ => Object::ERROR(format!(
            "unknown operator: BOOLEAN {} BOOLEAN",
            operator
        )),
    }
}

fn eval_conditional_expression(conditional: &Conditional, env: &mut Environment) ->  Object {
    let condition = eval_expression(&conditional.condition, env);
    if is_error(&condition) {
        return condition;
    }
    if is_truthy(&condition) {
        eval_block_statemet(&conditional.consequence, env)
    } else if let Some(alternative) = &conditional.alternative {
        eval_block_statemet(alternative, env)
    } else {
        NULL
    }
}

fn is_truthy(object: &Object) -> bool {
    match object {
        Object::NULL => false,
        Object::BOOLEAN(x) => *x,
        _ => true,
    }
}

fn is_error(object: &Object) -> bool {
    match object {
        Object::ERROR(_) => true,
        _ => false,
    }
}

fn eval_identifier(identifier: &Identifier, env: &mut Environment) ->  Object {
    match env.get(&identifier.to_string()) {
        Some(x) => x.clone(),
        None => Object::ERROR(format!(
            "identifier not found: {}",
            identifier
        )),
    }
}

fn eval_expressions(expressions: &[Expression], env: &mut Environment) -> Vec<Object> {
    let mut result = vec![];
    for expression in expressions {
        let evaluated = eval_expression(expression, env);
        if is_error(&evaluated) {
            return vec![evaluated];
        }
        result.push(evaluated);
    }
    result
}

fn apply_function(function: &Object, args: &[Object]) ->  Object {
    match function {
        Object::FUNCTION(function) => {
            let mut extended_env = extend_function_env(function, args);
            let evaluated = eval_block_statemet(&function.body, &mut extended_env);
            return evaluated;
        }
        _ => Object::ERROR(format!("not a function: {}", function)),
    }
}

fn extend_function_env(function: &FunctionObject, args: &[Object]) -> Environment {
    let mut env = Environment::new_enclosed_environment(function.environment.clone());
    for (param, arg) in function.parameters.iter().zip(args) {
        env.set(param.to_string(), arg.clone());
    }
    env
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
            test_boolean_object(evaluated, expected);
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

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            ("if (10 > 1) { return 10; }", 10),
            ("if (10 > 1) { if (10 > 1) { return 10; } return 1; }", 10),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input.to_string());
            test_integer_object(evaluated, expected);
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -true"),
            ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                r#"
if (10 > 1) {
if (10 > 1) {
return true + false;
}
return 1;
}"#,
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "identifier not found: foobar"),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input.to_string());
            test_error_object(evaluated, expected.to_string());
        }
    }

    #[test]
    fn test_let_stateemtns() {
        let tests = vec![
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input.to_string());
            test_integer_object(evaluated, expected);
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2; };";

        let evaluated = test_eval(input.to_string());

        match evaluated {
            Object::FUNCTION(x) => {
                assert_eq!(x.parameters.len(), 1);
                assert_eq!(x.parameters[0].to_string(), "x");
                assert_eq!(x.body.to_string(), "(x + 2)\n");
            }
            _ => assert!(false, "The object is not a function"),
        }
    }

    #[test]
    fn test_function_application() {
        let tests = vec![
            ("let identity = fn(x) { x; }; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 11);", 16),
            (
                "let add = fn(x, y) { x + y; }; add(5 + 5, add(10, 10));",
                30,
            ),
            ("fn(x) { x; }(5)", 5),
        ];
        for (input, expected) in tests {
            let evaluated = test_eval(input.to_string());
            test_integer_object(evaluated, expected);
        }
    }

    #[test]
    fn test_closures() {
        let input = r#"
        let newAdder = fn(x) {
            fn(y) { x + y };
        };

        let addTwo = newAdder(2);
        addTwo(2);"#;

        test_integer_object(test_eval(input.to_string()), 4);
    }

    fn test_eval(input: String) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut env = Environment::new();
        eval_program(&program, &mut env)
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

    fn test_error_object(object: Object, expected: String) {
        match object {
            Object::ERROR(x) => assert_eq!(x, expected),
            _ => assert!(false, "The object is not an  error"),
        }
    }
}
