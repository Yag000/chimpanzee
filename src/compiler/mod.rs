use crate::{
    code::{Instructions, Opcode},
    evaluator::object::Object,
    parser::ast::{Expression, Primitive, Statement},
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
            Statement::Expression(s) => self.compile_expression(s),
            _ => unimplemented!(),
        }
    }

    fn compile_expression(&mut self, expression: Expression) -> Result<(), String> {
        match expression {
            Expression::Infix(infix) => {
                self.compile_expression(*infix.left)?;
                self.compile_expression(*infix.right)?;
                self.compile_infix_operator(infix.token)?;
                Ok(())
            }
            Expression::Primitive(primitive) => self.compile_primitive(primitive),
            _ => unimplemented!(),
        }
    }

    fn compile_primitive(&mut self, primitive: Primitive) -> Result<(), String> {
        match primitive {
            Primitive::IntegerLiteral(i) => {
                let integer = Object::INTEGER(i);
                let pos = self.add_constant(integer);
                self.emit(Opcode::Constant, vec![pos]);
                Ok(())
            }
            _ => unimplemented!(),
        }
    }

    fn compile_infix_operator(&mut self, operator: Token) -> Result<(), String> {
        match operator {
            Token::Plus => self.emit(Opcode::Add, vec![]),
            _ => return Err(format!("Unknown operator: {:?}", operator)),
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
        let tests = vec![CompilerTestCase {
            input: "1 + 2".to_string(),
            expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
            expected_instructions: flatten_instructions(vec![
                Opcode::Constant.make(vec![0]),
                Opcode::Constant.make(vec![1]),
                Opcode::Add.make(vec![]),
            ]),
        }];

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
                    test_instructions(&bytecode.instructions, &test.expected_instructions);
                    test_constants(
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

    pub fn test_instructions(instructions: &Instructions, expected: &Instructions) {
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

    pub fn test_constants(constants: &Vec<Object>, expected: &Vec<Rc<Object>>) {
        assert_eq!(
            constants.len(),
            expected.len(),
            "wrong number of constants. got={:?}, want={:?}",
            constants.len(),
            expected.len()
        );

        for (i, constant) in constants.iter().enumerate() {
            match constant {
                Object::INTEGER(x) => test_integer_object(x, &expected[i]),
                _ => panic!("constant[{}] - wrong type. got={:?}", i, constant),
            }
        }
    }

    pub fn test_integer_object(integer: &i64, expected: &Object) {
        match expected {
            Object::INTEGER(i) => assert_eq!(
                integer, i,
                "integer object has wrong value. got={}, want={}",
                integer, i
            ),
            _ => panic!("object is not Integer. got={:?}", expected),
        }
    }
}
