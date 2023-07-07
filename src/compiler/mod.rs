use crate::{
    code::{Instructions, Opcode},
    evaluator::object::Object,
    parser::ast::{Expression, InfixOperator, Primitive, Statement},
    Program, Token,
};

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Instructions::default(),
            constants: vec![],
        }
    }

    pub fn compile(&mut self, program: Program) -> Result<(), String> {
        for statement in program.statements {
            self.compile_statement(statement)?;
        }

        Ok(())
    }

    fn compile_statement(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::Expression(s) => {
                self.compile_expression(s)?;
                self.emit(Opcode::Pop, vec![]);
            }
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn compile_expression(&mut self, expression: Expression) -> Result<(), String> {
        match expression {
            Expression::Infix(infix) => match infix.token {
                Token::LT | Token::LTE => self.compile_lt_and_lte(infix)?,
                _ => {
                    self.compile_expression(*infix.left)?;
                    self.compile_expression(*infix.right)?;
                    self.compile_infix_operator(infix.token)?;
                }
            },
            Expression::Primitive(primitive) => self.compile_primitive(primitive)?,
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn compile_primitive(&mut self, primitive: Primitive) -> Result<(), String> {
        match primitive {
            Primitive::IntegerLiteral(i) => {
                let integer = Object::INTEGER(i);
                let pos = self.add_constant(integer);
                self.emit(Opcode::Constant, vec![pos]);
            }
            Primitive::BooleanLiteral(true) => {
                self.emit(Opcode::True, vec![]);
            }
            Primitive::BooleanLiteral(false) => {
                self.emit(Opcode::False, vec![]);
            }
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn compile_infix_operator(&mut self, operator: Token) -> Result<(), String> {
        match operator {
            Token::Plus => self.emit(Opcode::Add, vec![]),
            Token::Minus => self.emit(Opcode::Sub, vec![]),
            Token::Asterisk => self.emit(Opcode::Mul, vec![]),
            Token::Slash => self.emit(Opcode::Div, vec![]),
            Token::GT => self.emit(Opcode::GreaterThan, vec![]),
            Token::GTE => self.emit(Opcode::GreaterEqualThan, vec![]),
            Token::Equal => self.emit(Opcode::Equal, vec![]),
            Token::NotEqual => self.emit(Opcode::NotEqual, vec![]),
            Token::Or => self.emit(Opcode::Or, vec![]),
            Token::And => self.emit(Opcode::And, vec![]),
            _ => return Err(format!("Unknown operator: {operator}")),
        };
        Ok(())
    }

    fn compile_lt_and_lte(&mut self, infix: InfixOperator) -> Result<(), String> {
        self.compile_expression(*infix.right)?;
        self.compile_expression(*infix.left)?;
        match infix.token {
            Token::LT => self.emit(Opcode::GreaterThan, vec![]),
            Token::LTE => self.emit(Opcode::GreaterEqualThan, vec![]),
            tk => return Err(format!("Unknown operator: {tk}")),
        };
        Ok(())
    }

    fn add_constant(&mut self, obj: Object) -> i32 {
        self.constants.push(obj);
        (self.constants.len() - 1) as i32
    }

    fn emit(&mut self, opcode: Opcode, operands: Vec<i32>) -> i32 {
        let instruction = opcode.make(operands);
        self.add_instruction(instruction)
    }

    fn add_instruction(&mut self, instruction: Instructions) -> i32 {
        let pos_new_instruction = self.instructions.data.len();
        self.instructions.append(instruction);
        pos_new_instruction as i32
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode::new(self.instructions.clone(), self.constants.clone())
    }
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

impl Bytecode {
    fn new(instructions: Instructions, constants: Vec<Object>) -> Self {
        Bytecode {
            instructions,
            constants,
        }
    }
}

#[cfg(test)]
pub mod tests {

    use std::rc::Rc;

    use crate::{code::Opcode, Lexer, Parser, Program};

    use super::*;

    struct CompilerTestCase {
        input: String,
        expected_constants: Vec<Object>,
        expected_instructions: Instructions,
    }

    #[test]
    fn test_integer_arithemtic() {
        let tests = vec![
            CompilerTestCase {
                input: "1 + 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Add.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1; 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 * 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Mul.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 / 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Div.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 - 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Sub.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_boolean_expression() {
        let tests = vec![
            CompilerTestCase {
                input: "true".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::True.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "false".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::False.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_boolean_logic() {
        let tests = vec![
            CompilerTestCase {
                input: "1 > 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::GreaterThan.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 >= 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::GreaterEqualThan.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 < 2".to_string(),
                expected_constants: vec![Object::INTEGER(2), Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::GreaterThan.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 <= 2".to_string(),
                expected_constants: vec![Object::INTEGER(2), Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::GreaterEqualThan.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 == 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Equal.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "1 != 2".to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::NotEqual.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "true == false".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::True.make(vec![]),
                    Opcode::False.make(vec![]),
                    Opcode::Equal.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "true != false".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::True.make(vec![]),
                    Opcode::False.make(vec![]),
                    Opcode::NotEqual.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    fn flatten_instructions(instructions: Vec<Instructions>) -> Instructions {
        let mut res = Instructions::default();
        for instruction in instructions {
            res.append(instruction);
        }
        res
    }

    fn run_compiler(tests: Vec<CompilerTestCase>) {
        for test in tests {
            let program = parse(&test.input);

            let mut compiler = Compiler::new();

            match compiler.compile(program) {
                Ok(_) => {
                    let bytecode = compiler.bytecode();
                    println!(
                        "want {}, got {}",
                        test.expected_instructions, bytecode.instructions
                    );
                    check_instructions(&bytecode.instructions, &test.expected_instructions);
                    check_constants(
                        &bytecode.constants,
                        &test
                            .expected_constants
                            .iter()
                            .map(|x| Rc::new(x.clone()))
                            .collect(),
                    );
                }
                Err(err) => panic!("compiler error: {}", err),
            }
        }
    }

    pub fn parse(input: &str) -> Program {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program()
    }

    pub fn check_instructions(instructions: &Instructions, expected: &Instructions) {
        assert_eq!(
            instructions.data.len(),
            expected.data.len(),
            "wrong instructions length"
        );
        assert_eq!(
            instructions, expected,
            "wrong instructions. want={:?}, got={:?}",
            expected, instructions
        );
    }

    pub fn check_constants(constants: &Vec<Object>, expected: &Vec<Rc<Object>>) {
        assert_eq!(
            constants.len(),
            expected.len(),
            "wrong number of constants. got={:?}, want={:?}",
            constants.len(),
            expected.len()
        );

        for (i, constant) in constants.iter().enumerate() {
            match constant {
                Object::INTEGER(x) => check_integer_object(x, &expected[i]),
                Object::BOOLEAN(x) => check_boolean_object(x, &expected[i]),
                _ => panic!("constant[{}] - wrong type. got={:?}", i, constant),
            }
        }
    }

    pub fn check_integer_object(integer: &i64, expected: &Object) {
        match expected {
            Object::INTEGER(i) => assert_eq!(
                integer, i,
                "integer object has wrong value. got={}, want={}",
                integer, i
            ),
            _ => panic!("object is not Integer. got={:?}", expected),
        }
    }

    pub fn check_boolean_object(boolean: &bool, expected: &Object) {
        match expected {
            Object::BOOLEAN(b) => assert_eq!(
                boolean, b,
                "boolean object has wrong value. got={}, want={}",
                boolean, b
            ),
            _ => panic!("object is not Boolean. got={:?}", expected),
        }
    }
}
