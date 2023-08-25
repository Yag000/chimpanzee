use crate::{
    lexer::token::Token,
    object::{
        builtins::BuiltinFunction,
        enviroment::Environment,
        {Function, Object, FALSE, NULL, TRUE},
    },
    parser::ast::{
        BlockStatement, Conditional, Expression, HashMapLiteral, Identifier, IndexExpression,
        Primitive, Program, Statement,
    },
};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

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

    pub fn eval(&mut self, program: Program) -> Object {
        let mut result = NULL;
        for statement in program.statements {
            result = self.eval_statement(statement);
            match result {
                Object::RETURN(x) => return *x,
                Object::ERROR(x) => return Object::ERROR(x),
                _ => (),
            }
        }
        result
    }

    fn eval_block_statemet(&mut self, block: BlockStatement) -> Object {
        let mut result = NULL;
        for statement in block.statements {
            result = self.eval_statement(statement);
            match result {
                Object::RETURN(_) | Object::ERROR(_) => return result,
                _ => (),
            }
        }
        result
    }

    #[allow(clippy::match_wildcard_for_single_variants, unreachable_patterns)]
    fn eval_statement(&mut self, statement: Statement) -> Object {
        match statement {
            Statement::Expression(x) => self.eval_expression(x),
            Statement::Return(x) => {
                let value = self.eval_expression(x.return_value);
                if Self::is_error(&value) {
                    return value;
                }
                Object::RETURN(Box::new(value))
            }
            Statement::Let(x) => {
                let value = self.eval_expression(x.value);
                if Self::is_error(&value) {
                    return value;
                }
                self.env.borrow_mut().set(x.name.to_string(), value);
                NULL
            }
            Statement::While(stm) => {
                let mut result = NULL;
                while Self::is_truthy(&self.eval_expression(stm.condition.clone())) {
                    result = self.eval_block_statemet(stm.body.clone());
                    match result {
                        Object::RETURN(_) | Object::ERROR(_) => return result,
                        _ => (),
                    }
                }
                result
            }

            _ => unimplemented!(), // I have decided not to implement the rest of the expressions,
                                   // I will focus on the compiler
        }
    }

    #[allow(clippy::match_wildcard_for_single_variants, unreachable_patterns)]
    fn eval_expression(&mut self, expression: Expression) -> Object {
        match expression {
            Expression::Primitive(x) => Self::eval_primitive_expression(x),
            Expression::Prefix(operator) => {
                let right = self.eval_expression(*operator.right);
                if Self::is_error(&right) {
                    return right;
                }
                Self::eval_prefix_expression(&operator.token, &right)
            }
            Expression::Infix(operator) => {
                let left = self.eval_expression(*operator.left);
                if Self::is_error(&left) {
                    return left;
                }
                let right = self.eval_expression(*operator.right);
                if Self::is_error(&right) {
                    return right;
                }
                Self::eval_infix_expression(&operator.token, left, right)
            }
            Expression::Conditional(conditional) => self.eval_conditional_expression(conditional),
            Expression::Identifier(x) => self.eval_identifier(&x),
            Expression::FunctionLiteral(x) => {
                let parameters = x.parameters;
                let body = x.body;
                Object::FUNCTION(Function {
                    parameters,
                    body,
                    environment: Rc::clone(&self.env),
                })
            }
            Expression::FunctionCall(x) => {
                let function = self.eval_expression(*x.function);
                if Self::is_error(&function) {}
                let args = self.eval_expressions(x.arguments);
                if args.len() == 1 && Self::is_error(&args[0]) {
                    return args[0].clone();
                }
                self.apply_function(function, args)
            }
            Expression::ArrayLiteral(array) => {
                let elements = self.eval_expressions(array.elements);
                if elements.len() == 1 && Self::is_error(&elements[0]) {
                    return elements[0].clone();
                }
                Object::ARRAY(elements)
            }
            Expression::IndexExpression(index_expression) => {
                self.eval_index_expression(index_expression)
            }
            Expression::HashMapLiteral(hashmap) => self.eval_hashmap_literal(hashmap),
            _ => unimplemented!(), // I have decided not to implement the rest of the expressions,
                                   // I will focus on the compiler
        }
    }

    fn eval_primitive_expression(expression: Primitive) -> Object {
        match expression {
            Primitive::IntegerLiteral(x) => Object::INTEGER(x),
            Primitive::BooleanLiteral(x) => {
                if x {
                    TRUE
                } else {
                    FALSE
                }
            }
            Primitive::StringLiteral(s) => Object::STRING(s),
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
        if Self::is_truthy(right) {
            FALSE
        } else {
            TRUE
        }
    }

    fn eval_minus_operator_expression(right: &Object) -> Object {
        match right {
            Object::INTEGER(x) => Object::INTEGER(-x),
            _ => Object::ERROR(format!("unknown operator: -{right}")),
        }
    }

    fn eval_infix_expression(operator: &Token, left: Object, right: Object) -> Object {
        match (left, right) {
            (Object::INTEGER(x), Object::INTEGER(y)) => {
                Self::eval_integer_infix_expression(operator, x, y)
            }
            (Object::BOOLEAN(x), Object::BOOLEAN(y)) => {
                Self::eval_boolean_infix_expression(operator, x, y)
            }
            (Object::STRING(x), Object::STRING(y)) => {
                Self::eval_string_infix_expression(operator, x, &y)
            }
            (left, right) => Object::ERROR(format!(
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
            Token::LTE => Object::BOOLEAN(left <= right),
            Token::GTE => Object::BOOLEAN(left >= right),
            Token::Equal => Object::BOOLEAN(left == right),
            Token::NotEqual => Object::BOOLEAN(left != right),
            _ => Object::ERROR(format!("unknown operator: INTEGER {operator} INTEGER")),
        }
    }

    fn eval_boolean_infix_expression(operator: &Token, left: bool, right: bool) -> Object {
        match operator {
            Token::Equal => Object::BOOLEAN(left == right),
            Token::NotEqual => Object::BOOLEAN(left != right),
            Token::And => Object::BOOLEAN(left && right),
            Token::Or => Object::BOOLEAN(left || right),
            _ => Object::ERROR(format!("unknown operator: BOOLEAN {operator} BOOLEAN")),
        }
    }

    fn eval_string_infix_expression(operator: &Token, mut left: String, right: &str) -> Object {
        match operator {
            Token::Plus => {
                left.push_str(right);
                Object::STRING(left)
            }

            _ => Object::ERROR(format!("unknown operator: STRING {operator} STRING")),
        }
    }

    fn eval_conditional_expression(&mut self, conditional: Conditional) -> Object {
        let condition = self.eval_expression(*conditional.condition);
        if Self::is_error(&condition) {
            return condition;
        }
        if Self::is_truthy(&condition) {
            self.eval_block_statemet(conditional.consequence)
        } else if let Some(alternative) = conditional.alternative {
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

    fn eval_expressions(&mut self, expressions: Vec<Expression>) -> Vec<Object> {
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

    fn apply_function(&mut self, function: Object, args: Vec<Object>) -> Object {
        match function {
            Object::FUNCTION(function) => {
                let extended_env = Self::extend_function_env(&function, args);
                let env = Rc::clone(&self.env);
                self.env = Rc::new(RefCell::new(extended_env));
                let evaluated = self.eval_block_statemet(function.body);
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

    fn eval_index_expression(&mut self, index_expression: IndexExpression) -> Object {
        let left = self.eval_expression(*index_expression.left);
        if Self::is_error(&left) {
            return left;
        }
        let index = self.eval_expression(*index_expression.index);
        if Self::is_error(&index) {
            return index;
        }
        match (&left, &index) {
            (Object::ARRAY(x), Object::INTEGER(y)) => {
                if *y < 0 || *y >= x.len() as i64 {
                    return NULL;
                }
                let index = usize::try_from(*y).unwrap();
                x[index].clone()
            }
            (Object::HASHMAP(x), _) => {
                if !index.is_hashable() {
                    return Object::ERROR(format!("unusable as hash key: {}", index.get_type()));
                }
                match x.get(&index) {
                    Some(x) => x.clone(),
                    None => NULL,
                }
            }

            _ => Object::ERROR(format!(
                "index operator not supported: {}[{}]",
                left.get_type(),
                index.get_type()
            )),
        }
    }

    fn eval_hashmap_literal(&mut self, hashmap_pairs: HashMapLiteral) -> Object {
        let mut hashmap = HashMap::new();
        for (key, value) in hashmap_pairs.pairs {
            let key = self.eval_expression(key);
            if Self::is_error(&key) {
                return key;
            }
            if !key.is_hashable() {
                return Object::ERROR(format!("unusable as hash key: {}", key.get_type()));
            }

            let value = self.eval_expression(value);
            if Self::is_error(&value) {
                return value;
            }
            hashmap.insert(key, value);
        }
        Object::HASHMAP(hashmap)
    }
}
