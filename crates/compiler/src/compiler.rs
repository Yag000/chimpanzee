use crate::code::{Instructions, Opcode};
use crate::symbol_table::SymbolTable;
use interpreter::object::Object;
use lexer::token::Token;
use num_traits::FromPrimitive;
use parser::ast::Program;
use parser::ast::{BlockStatement, Conditional, Expression, InfixOperator, Primitive, Statement};

pub struct Compiler {
    instructions: Instructions,
    pub constants: Vec<Object>,

    last_instruction: Option<EmittedInstruction>,
    previous_instruction: Option<EmittedInstruction>,

    pub symbol_table: SymbolTable,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct EmittedInstruction {
    opcode: Opcode,
    position: usize,
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

            last_instruction: None,
            previous_instruction: None,

            symbol_table: SymbolTable::default(),
        }
    }

    pub fn new_with_state(symbol_table: SymbolTable, constants: Vec<Object>) -> Self {
        let mut compiler = Compiler::new();
        compiler.symbol_table = symbol_table;
        compiler.constants = constants;
        compiler
    }

    pub fn compile(&mut self, program: Program) -> Result<(), String> {
        self.compile_statements(program.statements)
    }

    fn compile_block_statement(&mut self, block: BlockStatement) -> Result<(), String> {
        self.compile_statements(block.statements)
    }

    fn compile_statements(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
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
            Statement::Let(s) => {
                self.compile_expression(s.value)?;

                let symbol = self.symbol_table.define(s.name.value);
                self.emit(Opcode::SetGlobal, vec![symbol.index as i32]);
            }
            Statement::Return(_) => unimplemented!(),
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
                    self.compile_infix_operator(&infix.token)?;
                }
            },
            Expression::Prefix(prefix) => {
                self.compile_expression(*prefix.right)?;
                self.compile_prefix_operator(&prefix.token)?;
            }
            Expression::Primitive(primitive) => self.compile_primitive(primitive)?,
            Expression::Conditional(conditional) => self.compile_conditional(conditional)?,
            Expression::Identifier(ident) => {
                let symbol = self.symbol_table.resolve(&ident.value);
                match symbol {
                    Some(symbol) => {
                        self.emit(Opcode::GetGlobal, vec![symbol.index as i32]);
                    }
                    None => {
                        return Err(format!("Undefined variable: {}", ident.value));
                    }
                }
            }
            Expression::ArrayLiteral(array) => {
                let len = i32::from_usize(array.elements.len()).ok_or("Invalid array length")?;
                for element in array.elements {
                    self.compile_expression(element)?;
                }
                self.emit(Opcode::Array, vec![len]);
            }
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn compile_primitive(&mut self, primitive: Primitive) -> Result<(), String> {
        match primitive {
            Primitive::IntegerLiteral(i) => {
                let integer = Object::INTEGER(i);
                let pos = self.add_constant(integer);
                let pos = i32::from_usize(pos).ok_or("Invalid constant position")?;
                self.emit(Opcode::Constant, vec![pos]);
            }
            Primitive::BooleanLiteral(true) => {
                self.emit(Opcode::True, vec![]);
            }
            Primitive::BooleanLiteral(false) => {
                self.emit(Opcode::False, vec![]);
            }
            Primitive::StringLiteral(s) => {
                let string = Object::STRING(s);
                let pos = self.add_constant(string);
                let pos = i32::from_usize(pos).ok_or("Invalid constant position")?;
                self.emit(Opcode::Constant, vec![pos]);
            }
        }

        Ok(())
    }

    fn compile_infix_operator(&mut self, operator: &Token) -> Result<(), String> {
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

    fn compile_prefix_operator(&mut self, operator: &Token) -> Result<(), String> {
        match operator {
            Token::Bang => self.emit(Opcode::Bang, vec![]),
            Token::Minus => self.emit(Opcode::Minus, vec![]),
            _ => return Err(format!("Unknown operator: {operator}")),
        };
        Ok(())
    }

    fn compile_conditional(&mut self, conditional: Conditional) -> Result<(), String> {
        self.compile_expression(*conditional.condition)?;

        let jump_not_truthy_pos = self.emit(Opcode::JumpNotTruthy, vec![9999]); // We emit a dummy value for the jump offset
                                                                                // and we will fix it later
        self.compile_block_statement(conditional.consequence)?;
        if self.is_last_instruction(Opcode::Pop) {
            self.remove_last_instruction();
        }

        let jump_pos = self.emit(Opcode::Jump, vec![9999]); // We emit a dummy value for the jump offset
                                                            // and we will fix it later

        let after_consequence_pos = self.instructions.data.len();
        self.change_operand(jump_not_truthy_pos, after_consequence_pos as i32)?;

        if let Some(alternative) = conditional.alternative {
            self.compile_block_statement(alternative)?;
            if self.is_last_instruction(Opcode::Pop) {
                self.remove_last_instruction();
            }
        } else {
            self.emit(Opcode::Null, vec![]);
        }

        let after_alternative_pos = self.instructions.data.len();
        self.change_operand(jump_pos, after_alternative_pos as i32)?;

        Ok(())
    }

    fn is_last_instruction(&self, opcode: Opcode) -> bool {
        match self.last_instruction {
            Some(ref last) => last.opcode == opcode,
            None => false,
        }
    }

    fn remove_last_instruction(&mut self) {
        if self.last_instruction.is_some() {
            self.instructions.data.pop();
            self.last_instruction = self.previous_instruction.clone();
        }
    }

    fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    fn emit(&mut self, opcode: Opcode, operands: Vec<i32>) -> usize {
        let instruction = opcode.make(operands);
        let pos = self.add_instruction(instruction);
        self.set_last_instruction(opcode, pos);
        pos
    }

    fn add_instruction(&mut self, instruction: Instructions) -> usize {
        let pos_new_instruction = self.instructions.data.len();
        self.instructions.append(instruction);
        pos_new_instruction
    }

    fn set_last_instruction(&mut self, opcode: Opcode, pos: usize) {
        let last = EmittedInstruction {
            opcode,
            position: pos,
        };
        self.previous_instruction = self.last_instruction.clone();
        self.last_instruction = Some(last);
    }

    fn change_operand(&mut self, pos: usize, operand: i32) -> Result<(), String> {
        let op = Opcode::from_u8(self.instructions.data[pos]).ok_or(format!(
            "Unknown opcode: {opcode}",
            opcode = self.instructions.data[pos]
        ))?;
        let new_instruction = op.make(vec![operand]);
        self.replace_instruction(pos, &new_instruction);
        Ok(())
    }

    fn replace_instruction(&mut self, pos: usize, new_instruction: &Instructions) {
        for (i, instruction) in new_instruction.data.iter().enumerate() {
            self.instructions.data[pos + i] = *instruction;
        }
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

#[allow(clippy::too_many_lines)]
#[cfg(test)]
pub mod tests {

    use std::rc::Rc;

    use crate::{
        code::Opcode,
        test_utils::{check_constants, check_instructions, parse},
    };

    use super::*;

    struct CompilerTestCase {
        input: String,
        expected_constants: Vec<Object>,
        expected_instructions: Instructions,
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
                Err(err) => panic!("compiler error: {err}"),
            }
        }
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
            CompilerTestCase {
                input: "-1".to_string(),
                expected_constants: vec![Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Minus.make(vec![]),
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
            CompilerTestCase {
                input: "!true".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::True.make(vec![]),
                    Opcode::Bang.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "!false".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::False.make(vec![]),
                    Opcode::Bang.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_conditionals() {
        let tests = vec![
            CompilerTestCase {
                input: "if (true) { 10 }; 3333;".to_string(),
                expected_constants: vec![Object::INTEGER(10), Object::INTEGER(3333)],
                expected_instructions: flatten_instructions(vec![
                    // 0000
                    Opcode::True.make(vec![]),
                    // 0001
                    Opcode::JumpNotTruthy.make(vec![10]),
                    // 0004
                    Opcode::Constant.make(vec![0]),
                    // 0007
                    Opcode::Jump.make(vec![11]),
                    // 0010
                    Opcode::Null.make(vec![]),
                    // 0011
                    Opcode::Pop.make(vec![]),
                    // 0012
                    Opcode::Constant.make(vec![1]),
                    // 0015
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "if (true) { 10 } else { 20 }; 3333;".to_string(),
                expected_constants: vec![
                    Object::INTEGER(10),
                    Object::INTEGER(20),
                    Object::INTEGER(3333),
                ],
                expected_instructions: flatten_instructions(vec![
                    // 0000
                    Opcode::True.make(vec![]),
                    // 0001
                    Opcode::JumpNotTruthy.make(vec![10]),
                    // 0004
                    Opcode::Constant.make(vec![0]),
                    // 0007
                    Opcode::Jump.make(vec![13]),
                    // 0010
                    Opcode::Constant.make(vec![1]),
                    // 0013
                    Opcode::Pop.make(vec![]),
                    // 0014
                    Opcode::Constant.make(vec![2]),
                    // 0017
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }
    #[test]
    fn test_let_statements() {
        let tests = vec![
            CompilerTestCase {
                input: r#"
                let one = 1;    
                let two = 2"#
                    .to_string(),
                expected_constants: vec![Object::INTEGER(1), Object::INTEGER(2)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::SetGlobal.make(vec![1]),
                ]),
            },
            CompilerTestCase {
                input: r#"
                let one = 1;
                one;"#
                    .to_string(),
                expected_constants: vec![Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::GetGlobal.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: r#"
                let one = 1;
                let two = one;
                two;"#
                    .to_string(),
                expected_constants: vec![Object::INTEGER(1)],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::SetGlobal.make(vec![0]),
                    Opcode::GetGlobal.make(vec![0]),
                    Opcode::SetGlobal.make(vec![1]),
                    Opcode::GetGlobal.make(vec![1]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_string_expressions() {
        let tests = vec![
            CompilerTestCase {
                input: r#""monkey""#.to_string(),
                expected_constants: vec![Object::STRING("monkey".to_string())],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: r#""mon" + "key""#.to_string(),
                expected_constants: vec![
                    Object::STRING("mon".to_string()),
                    Object::STRING("key".to_string()),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Add.make(vec![]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }

    #[test]
    fn test_array_expressions() {
        let tests = vec![
            CompilerTestCase {
                input: "[]".to_string(),
                expected_constants: vec![],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Array.make(vec![0]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "[1, 2, 3]".to_string(),
                expected_constants: vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::INTEGER(3),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Constant.make(vec![2]),
                    Opcode::Array.make(vec![3]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
            CompilerTestCase {
                input: "[1 + 2, 3 - 4, 5 * 6]".to_string(),
                expected_constants: vec![
                    Object::INTEGER(1),
                    Object::INTEGER(2),
                    Object::INTEGER(3),
                    Object::INTEGER(4),
                    Object::INTEGER(5),
                    Object::INTEGER(6),
                ],
                expected_instructions: flatten_instructions(vec![
                    Opcode::Constant.make(vec![0]),
                    Opcode::Constant.make(vec![1]),
                    Opcode::Add.make(vec![]),
                    Opcode::Constant.make(vec![2]),
                    Opcode::Constant.make(vec![3]),
                    Opcode::Sub.make(vec![]),
                    Opcode::Constant.make(vec![4]),
                    Opcode::Constant.make(vec![5]),
                    Opcode::Mul.make(vec![]),
                    Opcode::Array.make(vec![3]),
                    Opcode::Pop.make(vec![]),
                ]),
            },
        ];

        run_compiler(tests);
    }
}
