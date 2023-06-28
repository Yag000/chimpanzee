pub mod enviroment;
pub mod object;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    parser::ast::{
        BlockStatement, Conditional, Expression, HashMapLiteral, Identifier, IndexExpression,
        Primitive, Statement,
    },
    Program, Token,
};

use self::{
    enviroment::Environment,
    object::{BuiltinFunction, Function, Object},
};

const TRUE: Object = Object::BOOLEAN(true);
const FALSE: Object = Object::BOOLEAN(false);
const NULL: Object = Object::NULL;

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn eval(&mut self, program: &Program) -> Object {
        let mut result = NULL;
        for statement in &program.statements {
            result = self.eval_statement(statement);
            match result {
                Object::RETURN(x) => return *x,
                Object::ERROR(x) => return Object::ERROR(x),
                _ => (),
            }
        }
        result
    }

    fn eval_block_statemet(&mut self, block: &BlockStatement) -> Object {
        let mut result = NULL;
        for statement in &block.statements {
            result = self.eval_statement(statement);
            match result {
                Object::RETURN(_) | Object::ERROR(_) => return result,
                _ => (),
            }
        }
        result
    }

    fn eval_statement(&mut self, statement: &Statement) -> Object {
        match statement {
            Statement::Expression(x) => self.eval_expression(x),
            Statement::Return(x) => {
                let value = self.eval_expression(&x.return_value);
                if Self::is_error(&value) {
                    return value;
                }
                Object::RETURN(Box::new(value))
            }
            Statement::Let(x) => {
                let value = self.eval_expression(&x.value);
                if Self::is_error(&value) {
                    return value;
                }
                self.env.borrow_mut().set(x.name.to_string(), value.clone());
                value
            }
        }
    }

    fn eval_expression(&mut self, expression: &Expression) -> Object {
        match expression {
            Expression::Primitive(x) => Self::eval_primitive_expression(x),
            Expression::Prefix(operator) => {
                let right = self.eval_expression(&operator.right);
                if Self::is_error(&right) {
                    return right;
                }
                Self::eval_prefix_expression(&operator.token, &right)
            }
            Expression::Infix(operator) => {
                let left = self.eval_expression(&operator.left);
                if Self::is_error(&left) {
                    return left;
                }
                let right = self.eval_expression(&operator.right);
                if Self::is_error(&right) {
                    return right;
                }
                Self::eval_infix_expression(&operator.token, &left, &right)
            }
            Expression::Conditional(conditional) => self.eval_conditional_expression(conditional),
            Expression::Identifier(x) => self.eval_identifier(x),
            Expression::FunctionLiteral(x) => {
                let parameters = &x.parameters;
                let body = &x.body;
                Object::FUNCTION(Function {
                    parameters: parameters.clone(),
                    body: body.clone(),
                    environment: Rc::clone(&self.env),
                })
            }
            Expression::FunctionCall(x) => {
                let function = self.eval_expression(&x.function);
                if Self::is_error(&function) {}
                let args = self.eval_expressions(&x.arguments);
                if args.len() == 1 && Self::is_error(&args[0]) {
                    return args[0].clone();
                }
                self.apply_function(&function, args)
            }
            Expression::ArrayLiteral(array) => {
                let elements = self.eval_expressions(&array.elements);
                if elements.len() == 1 && Self::is_error(&elements[0]) {
                    return elements[0].clone();
                }
                Object::ARRAY(elements)
            }
            Expression::IndexExpression(index_expression) => {
                self.eval_index_expression(index_expression)
            }
            Expression::HashMapLiteral(hashmap) => self.eval_hashmap_literal(hashmap),
        }
    }

    fn eval_primitive_expression(expression: &Primitive) -> Object {
        match expression {
            Primitive::IntegerLiteral(x) => Object::INTEGER(*x),
            Primitive::BooleanLiteral(x) => {
                if *x {
                    TRUE
                } else {
                    FALSE
                }
            }
            Primitive::StringLiteral(s) => Object::STRING(s.clone()),
        }
    }

    fn eval_prefix_expression(operator: &Token, right: &Object) -> Object {
        match operator {
            Token::Bang => Self::eval_bang_operator_expression(right),
            Token::Minus => Self::eval_minus_operator_expression(right),
            _ => Object::ERROR(format!("unknown operator: {operator}{right}")),
        }
    }

    fn eval_bang_operator_expression(right: &Object) -> Object {
        match right {
            Object::BOOLEAN(false) | Object::NULL => TRUE,
            _ => FALSE,
        }
    }

    fn eval_minus_operator_expression(right: &Object) -> Object {
        match right {
            Object::INTEGER(x) => Object::INTEGER(-x),
            _ => Object::ERROR(format!("unknown operator: -{right}")),
        }
    }

    fn eval_infix_expression(operator: &Token, left: &Object, right: &Object) -> Object {
        match (left, right) {
            (Object::INTEGER(x), Object::INTEGER(y)) => {
                Self::eval_integer_infix_expression(operator, *x, *y)
            }
            (Object::BOOLEAN(x), Object::BOOLEAN(y)) => {
                Self::eval_boolean_infix_expression(operator, *x, *y)
            }
            (Object::STRING(x), Object::STRING(y)) => {
                Self::eval_string_infix_expression(operator, x, y)
            }
            _ => Object::ERROR(format!(
                "type mismatch: {} {} {}",
                left.get_type(),
                operator,
                right.get_type()
            )),
        }
    }

    fn eval_integer_infix_expression(operator: &Token, left: i64, right: i64) -> Object {
        match operator {
            Token::Plus => Object::INTEGER(left + right),
            Token::Minus => Object::INTEGER(left - right),
            Token::Asterisk => Object::INTEGER(left * right),
            Token::Slash => Object::INTEGER(left / right),
            Token::LT => Object::BOOLEAN(left < right),
            Token::GT => Object::BOOLEAN(left > right),
            Token::Equal => Object::BOOLEAN(left == right),
            Token::NotEqual => Object::BOOLEAN(left != right),
            _ => Object::ERROR(format!("unknown operator: INTEGER {operator} INTEGER")),
        }
    }

    fn eval_boolean_infix_expression(operator: &Token, left: bool, right: bool) -> Object {
        match operator {
            Token::Equal => Object::BOOLEAN(left == right),
            Token::NotEqual => Object::BOOLEAN(left != right),
            _ => Object::ERROR(format!("unknown operator: BOOLEAN {operator} BOOLEAN")),
        }
    }

    fn eval_string_infix_expression(operator: &Token, left: &str, right: &str) -> Object {
        match operator {
            Token::Plus => Object::STRING(format!("{left}{right}")),
            _ => Object::ERROR(format!("unknown operator: STRING {operator} STRING")),
        }
    }

    fn eval_conditional_expression(&mut self, conditional: &Conditional) -> Object {
        let condition = self.eval_expression(&conditional.condition);
        if Self::is_error(&condition) {
            return condition;
        }
        if Self::is_truthy(&condition) {
            self.eval_block_statemet(&conditional.consequence)
        } else if let Some(alternative) = &conditional.alternative {
            self.eval_block_statemet(alternative)
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
        matches!(object, Object::ERROR(_))
    }

    fn eval_identifier(&self, identifier: &Identifier) -> Object {
        match self.env.borrow().get(&identifier.to_string()) {
            Some(x) => x,
            None => match BuiltinFunction::get_builtin(&identifier.to_string()) {
                Some(x) => x,
                None => Object::ERROR(format!("identifier not found: {identifier}")),
            },
        }
    }

    fn eval_expressions(&mut self, expressions: &[Expression]) -> Vec<Object> {
        let mut result = vec![];
        for expression in expressions {
            let evaluated = self.eval_expression(expression);
            if Self::is_error(&evaluated) {
                return vec![evaluated];
            }
            result.push(evaluated);
        }
        result
    }

    fn apply_function(&mut self, function: &Object, args: Vec<Object>) -> Object {
        match function {
            Object::FUNCTION(function) => {
                let extended_env = Self::extend_function_env(function, args);
                let env = Rc::clone(&self.env);
                self.env = Rc::new(RefCell::new(extended_env));
                let evaluated = self.eval_block_statemet(&function.body);
                self.env = env;
                evaluated
            }
            Object::BUILTIN(function) => function.call(args),
            _ => Object::ERROR(format!("not a function: {function}")),
        }
    }

    fn extend_function_env(function: &Function, args: Vec<Object>) -> Environment {
        let mut env = Environment::new_enclosed_environment(Rc::clone(&function.environment));
        for (param, arg) in function.parameters.iter().zip(args) {
            env.set(param.to_string(), arg);
        }
        env
    }

    fn eval_index_expression(&mut self, index_expression: &IndexExpression) -> Object {
        let left = self.eval_expression(&index_expression.left);
        if Self::is_error(&left) {
            return left;
        }
        let index = self.eval_expression(&index_expression.index);
        if Self::is_error(&index) {
            return index;
        }
        match (&left, &index) {
            (Object::ARRAY(x), Object::INTEGER(y)) => {
                if *y < 0 || *y >= x.len() as i64 {
                    return NULL;
                }
                x[*y as usize].clone()
            }
            (Object::HASHMAP(x), _) => self.eval_hashmap_index_expression(x, &index),
            _ => Object::ERROR(format!(
                "index operator not supported: {}[{}]",
                left.get_type(),
                index.get_type()
            )),
        }
    }
    fn eval_hashmap_index_expression(
        &self,
        hashmap: &HashMap<Object, Object>,
        index: &Object,
    ) -> Object {
        if !index.is_hashable() {
            return Object::ERROR(format!("unusable as hash key: {}", index.get_type()));
        }
        match hashmap.get(index) {
            Some(x) => x.clone(),
            None => NULL,
        }
    }
    fn eval_hashmap_literal(&mut self, hashmap_pairs: &HashMapLiteral) -> Object {
        let mut hashmap = HashMap::new();
        for (key, value) in hashmap_pairs.pairs.clone() {
            let key = self.eval_expression(&key);
            if Self::is_error(&key) {
                return key;
            }
            if !key.is_hashable() {
                return Object::ERROR(format!("unusable as hash key: {}", key.get_type()));
            }

            let value = self.eval_expression(&value);
            if Self::is_error(&value) {
                return value;
            }
            hashmap.insert(key, value);
        }
        Object::HASHMAP(hashmap)
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Lexer, Parser};
    use std::collections::HashMap;

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
            let evaluated = test_eval(input);
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
            let evaluated = test_eval(input);
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
            let evaluated = test_eval(input);
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
            let evaluated = test_eval(input);
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
            let evaluated = test_eval(input);
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
            (r#""Hello" - "World""#, "unknown operator: STRING - STRING"),
            (
                r#"{"name": "Monkey"}[fn(x) { x }];"#,
                "unusable as hash key: FUNCTION",
            ),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
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
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2; };";

        let evaluated = test_eval(input);

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
            let evaluated = test_eval(input);
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

        test_integer_object(test_eval(input), 4);
    }

    #[test]
    fn test_string_literal() {
        let input = "\"Hello World!\"";

        let evaluated = test_eval(input);

        test_string_object(evaluated, "Hello World!".to_string());
    }

    #[test]
    fn test_string_concatenationm() {
        let input = "\"Hello\" + \" \" + \"World!\"";

        let evaluated = test_eval(input.clone());

        test_string_object(evaluated, "Hello World!".to_string());
    }

    #[test]
    fn test_builttin_len_function() {
        let tests_striung = vec![
            (r#"len("")"#, 0),
            (r#"len("four")"#, 4),
            (r#"len("hello world")"#, 11),
            (r#"len([1,2,3,4,5])"#, 5),
        ];

        for (input, expected) in tests_striung {
            test_integer_object(test_eval(input), expected)
        }
    }

    #[test]
    fn test_builttin_len_function_errors() {
        let tests_striung = vec![
            (r#"len(1)"#, "argument to `len` not supported, got INTEGER"),
            (
                r#"len("one", "two")"#,
                "wrong number of arguments. got=2, want=1",
            ),
        ];

        for (input, expected) in tests_striung {
            test_error_object(test_eval(input), expected.to_string());
        }
    }

    #[test]
    fn test_array_literals() {
        let input = "[1, 2 * 2, 3 + 3]";

        let evaluated = test_eval(input);

        match evaluated {
            Object::ARRAY(x) => {
                assert_eq!(x.len(), 3);
                test_integer_object(x[0].clone(), 1);
                test_integer_object(x[1].clone(), 4);
                test_integer_object(x[2].clone(), 6);
            }
            _ => assert!(false, "The object is not an array"),
        }
    }

    #[test]
    fn test_array_index_expression() {
        let tests = vec![
            ("[1, 2, 3][0]", Some(1)),
            ("[1, 2, 3][1]", Some(2)),
            ("[1, 2, 3][2]", Some(3)),
            ("let i = 0; [1][i];", Some(1)),
            ("[1, 2, 3][1 + 1];", Some(3)),
            ("let myArray = [1, 2, 3]; myArray[2];", Some(3)),
            (
                "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
                Some(6),
            ),
            (
                "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
                Some(2),
            ),
            ("[1, 2, 3][3]", None),
            ("[1, 2, 3][-1]", None),
        ];

        for (input, expected) in tests {
            match expected {
                Some(x) => test_integer_object(test_eval(input), x),
                None => test_null_object(test_eval(input)),
            }
        }
    }

    #[test]
    fn test_first_function() {
        let tests = vec![
            ("first([1, 2, 3])", Some(1)),
            ("first([1])", Some(1)),
            ("first([])", None),
            ("first(1)", None),
            ("first([1, 2, 3], [4, 5, 6])", None),
        ];

        for (input, expected) in tests {
            println!("{}", input);
            match expected {
                Some(x) => test_integer_object(test_eval(input), x),
                None => test_null_object(test_eval(input)),
            }
        }
    }

    #[test]
    fn test_last_function() {
        let tests = vec![
            ("last([1, 2, 3])", Some(3)),
            ("last([1])", Some(1)),
            ("last([])", None),
            ("last(1)", None),
            ("last([1, 2, 3], [4, 5, 6])", None),
        ];

        for (input, expected) in tests {
            println!("{}", input);
            match expected {
                Some(x) => test_integer_object(test_eval(input), x),
                None => test_null_object(test_eval(input)),
            }
        }
    }

    #[test]
    fn test_rest_function() {
        let tests = vec![
            ("rest([1, 2, 3])", Some(vec![2, 3])),
            ("rest([1])", Some(Vec::new())),
            ("rest([])", None),
            ("rest(1)", None),
            ("rest([1, 2, 3], [4, 5, 6])", None),
        ];

        for (input, expected) in tests {
            println!("{}", input);
            match expected {
                Some(x) => {
                    let evaluated = test_eval(input);
                    test_array_object(evaluated, x);
                }
                None => test_null_object(test_eval(input)),
            }
        }
    }

    #[test]
    fn test_push_function() {
        let tests = vec![
            ("push([], 1)", Some(vec![1])),
            ("push([1], 2)", Some(vec![1, 2])),
            ("push([1,2], 3)", Some(vec![1, 2, 3])),
            ("push(1, 1)", None),
            ("push([1,2], 3, 4)", None),
        ];

        for (input, expected) in tests {
            println!("{}", input);
            match expected {
                Some(x) => test_array_object(test_eval(input), x),
                None => test_null_object(test_eval(input)),
            }
        }
    }

    #[test]
    fn test_array_functions_together() {
        let input = r#"
        let map = fn(arr, f) {
            let iter = fn(arr, accumulated) {
                if (len(arr) == 0) {
                    accumulated
                } else {
                    iter(rest(arr), push(accumulated, f(first(arr))));
                }
            };
            iter(arr, []);
        };
        let a = [1, 2, 3, 4];
        let double = fn(x) { x * 2 };
        map(a, double);
        "#;

        let expected = vec![2, 4, 6, 8];

        test_array_object(test_eval(input), expected);
    }

    #[test]
    fn test_evaluate_hash_literals() {
        let input = r#"
        let two = "two";
        {
            "one": 10 - 9,
            two: 1 + 1,
            "thr" + "ee": 6 / 2,
            4: 4,
            true: 5,
            false: 6
        }
        "#;

        let mut expected = HashMap::new();
        expected.insert(Object::STRING("one".to_string()), Object::INTEGER(1));
        expected.insert(Object::STRING("two".to_string()), Object::INTEGER(2));
        expected.insert(Object::STRING("three".to_string()), Object::INTEGER(3));
        expected.insert(Object::INTEGER(4), Object::INTEGER(4));
        expected.insert(Object::BOOLEAN(true), Object::INTEGER(5));
        expected.insert(Object::BOOLEAN(false), Object::INTEGER(6));

        let evaluated = test_eval(input);
        match evaluated {
            Object::HASHMAP(hash) => {
                assert_eq!(hash.len(), expected.len());

                for (expected_key, expected_value) in expected {
                    match hash.get(&expected_key) {
                        Some(value) => assert_eq!(value, &expected_value),
                        None => assert!(false, "No pair for given key in Pairs"),
                    }
                }
            }
            _ => assert!(false, "The object is not a hash"),
        }
    }

    #[test]
    fn test_hash_index_expressions() {
        let tests = vec![
            (r#"{"foo": 5}["foo"]"#, Some(5)),
            (r#"{"foo": 5}["bar"]"#, None),
            (r#"let key = "foo"; {"foo": 5}[key]"#, Some(5)),
            (r#"{}["foo"]"#, None),
            (r#"{5: 5}[5]"#, Some(5)),
            (r#"{true: 5}[true]"#, Some(5)),
            (r#"{false: 5}[false]"#, Some(5)),
        ];

        for (input, expected) in tests {
            println!("{}", input);
            match expected {
                Some(x) => test_integer_object(test_eval(input), x),
                None => test_null_object(test_eval(input)),
            }
        }
    }

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new();
        evaluator.eval(&program)
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
            Object::NULL | Object::ERROR(_) => (),

            _ => assert!(false, "The object is not null"),
        }
    }

    fn test_error_object(object: Object, expected: String) {
        match object {
            Object::ERROR(x) => assert_eq!(x, expected),
            _ => assert!(false, "The object is not an  error"),
        }
    }

    fn test_string_object(object: Object, expected: String) {
        match object {
            Object::STRING(s) => assert_eq!(format!("{s}"), expected),
            _ => assert!(false, "The object is not an string"),
        }
    }

    fn test_array_object(object: Object, expected: Vec<i64>) {
        match object {
            Object::ARRAY(x) => {
                assert_eq!(x.len(), expected.len());
                for (i, v) in x.iter().enumerate() {
                    test_integer_object(v.clone(), expected[i]);
                }
            }
            _ => assert!(false, "The object is not an array"),
        }
    }
}
