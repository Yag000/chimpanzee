use crate::parser::{
    ast::{BlockStatement, Expression, FunctionLiteral, Precedence, Program, Statement},
    parser::parse,
};

/// A formatter function scope.
///
/// This is used to keep track of the current function being formatted.
/// In particular it is used to determine if a semicolon should be added
/// to the end of a statement, due to implicit return rules.
#[derive(Debug, Clone)]
struct FormatterFunctionScope {
    outer: Option<Box<FormatterFunctionScope>>,
    block_statement_length: usize,
    current_position: usize,
}

impl FormatterFunctionScope {
    fn new(outer: Option<Box<FormatterFunctionScope>>, block_statement_length: usize) -> Self {
        Self {
            outer,
            block_statement_length,
            current_position: 0,
        }
    }

    fn leave_scope(&mut self) -> Option<Box<FormatterFunctionScope>> {
        self.outer.take()
    }

    fn next(&mut self) {
        self.current_position += 1;
    }

    fn is_end(&self) -> bool {
        self.current_position == self.block_statement_length - 1
    }
}

pub struct Formatter {
    /// The current indentation level.
    indent: usize,

    /// Current precedence.
    preference: Precedence,

    /// Previous expression on the ast
    last_expression: Option<Expression>,

    /// The current formatter function scope.
    formatter_function_scope: Option<Box<FormatterFunctionScope>>,

    /// The output buffer.
    output: String,
}

impl Formatter {
    fn new() -> Self {
        Self {
            indent: 0,
            preference: Precedence::Lowest,
            last_expression: None,
            formatter_function_scope: None,
            output: String::new(),
        }
    }

    pub fn format(input: &str) -> String {
        let program = parse(input);
        Self::format_program(program)
    }

    pub fn format_program(program: Program) -> String {
        let mut formatter = Self::new();

        formatter.visit_program(program);
        formatter.output.clone()
    }

    fn visit_program(&mut self, program: Program) {
        for stmt in program.statements {
            self.visit_statement(&stmt);
        }
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        self.push_indent();
        match stmt {
            Statement::Let(let_stmt) => {
                self.push("let ");
                self.push(let_stmt.name.value.as_str());
                self.push(" = ");
                self.visit_expression(&let_stmt.value);
                self.push(";");
            }
            Statement::Return(return_stmt) => {
                self.push("return ");
                self.visit_expression(&return_stmt.return_value);
                self.push(";");
            }
            Statement::Expression(exp_stmt) => {
                self.visit_expression(exp_stmt);
                if let Some(Expression::Conditional(_)) = self.last_expression {
                } else if self.formatter_function_scope.is_some() {
                    if !self.formatter_function_scope.clone().unwrap().is_end() {
                        self.push(";");
                    }
                } else {
                    self.push(";");
                }
            }
        }
        self.push("\n");
        self.last_expression = None;
    }

    fn visit_expression(&mut self, exp: &Expression) {
        match exp {
            Expression::Identifier(ident) => {
                self.push(ident.value.as_str());
            }
            Expression::Primitive(primitive) => {
                self.push(primitive.to_string().as_str());
            }
            Expression::Prefix(prefix) => {
                self.push(prefix.token.to_string().as_str());

                self.last_expression = Some(exp.clone());
                self.visit_expression(&prefix.right);
            }
            Expression::Infix(infix) => {
                let mut needs_parenthesis = false;
                if let Some(last) = &self.last_expression {
                    match &last {
                        Expression::Prefix(_) => {
                            self.push("(");
                            needs_parenthesis = true;
                        }
                        Expression::Infix(last_infix) => {
                            if Precedence::from(&last_infix.token) > Precedence::from(&infix.token)
                            {
                                self.push("(");
                                needs_parenthesis = true;
                            }
                        }
                        _ => {}
                    }
                }

                self.last_expression = Some(exp.clone());
                self.visit_expression(&infix.left);
                self.push(" ");
                self.push(infix.token.to_string().as_str());
                self.push(" ");

                self.last_expression = Some(exp.clone());
                self.visit_expression(&infix.right);

                if needs_parenthesis {
                    self.push(")");
                }
            }
            Expression::Conditional(if_exp) => {
                self.push("if (");

                self.last_expression = Some(exp.clone());
                self.visit_expression(&if_exp.condition);
                self.push(") {");
                self.push("\n");

                self.indent += 1;
                self.last_expression = Some(exp.clone());
                self.visit_block_statement(&if_exp.consequence);
                self.indent -= 1;

                self.push_indent();
                self.push("}");
                if let Some(alternative) = &if_exp.alternative {
                    self.push(" else {\n");
                    self.indent += 1;

                    self.last_expression = Some(exp.clone());
                    self.visit_block_statement(alternative);
                    self.indent -= 1;
                    self.push_indent();
                    self.push("}");
                }
            }
            Expression::FunctionLiteral(func) => self.visit_function_literal(func),
            Expression::FunctionCall(call) => {
                self.last_expression = Some(exp.clone());
                self.visit_expression(&call.function);
                self.push("(");
                for (i, arg) in call.arguments.iter().enumerate() {
                    self.last_expression = Some(exp.clone());
                    self.visit_expression(arg);
                    if i < call.arguments.len() - 1 {
                        self.push(", ");
                    }
                }
                self.push(")");
            }
            Expression::ArrayLiteral(array) => {
                self.push(array.to_string().as_str());
            }
            Expression::HashMapLiteral(hash) => {
                self.push(hash.to_string().as_str());
            }
            Expression::IndexExpression(index) => {
                self.last_expression = Some(exp.clone());
                self.visit_expression(&index.left);
                self.push("[");

                self.last_expression = Some(exp.clone());
                self.visit_expression(&index.index);
                self.push("]");
            }
        }

        self.last_expression = Some(exp.clone());

        self.preference = self.get_precedence(exp);
    }

    fn visit_function_literal(&mut self, func: &FunctionLiteral) {
        self.push("fn (");
        let parameters = func
            .parameters
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        self.push(parameters.join(", ").as_str());
        self.push(") {");
        self.push("\n");

        self.enter_function(func);
        for stmt in &func.body.statements {
            self.visit_statement(stmt);
            self.formatter_function_scope.as_mut().unwrap().next();
        }
        self.leave_function();

        self.push_indent();
        self.push("}");
    }

    fn visit_block_statement(&mut self, block: &BlockStatement) {
        for stmt in &block.statements {
            self.visit_statement(stmt);
        }
    }

    fn get_precedence(&self, exp: &Expression) -> Precedence {
        match exp {
            Expression::Infix(infix) => Precedence::from(&infix.token),
            Expression::Prefix(prefix) => Precedence::from(&prefix.token),
            _ => Precedence::Lowest,
        }
    }

    fn enter_function(&mut self, function: &FunctionLiteral) {
        self.formatter_function_scope = Some(Box::new(FormatterFunctionScope::new(
            self.formatter_function_scope.clone(),
            function.body.statements.len(),
        )));

        self.indent += 1;
    }

    fn leave_function(&mut self) {
        self.indent -= 1;
        if let Some(ref mut scope) = self.formatter_function_scope {
            self.formatter_function_scope = scope.leave_scope();
        }
    }

    fn push(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn push_indent(&mut self) {
        for _ in 0..self.indent {
            self.push("    ");
        }
    }
}
