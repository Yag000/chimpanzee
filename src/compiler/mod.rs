pub mod code;
mod compiler_tests;
pub mod symbol_table;
mod test_utils;

use std::{cell::RefCell, rc::Rc};

use crate::{
    compiler::{
        code::{Instructions, Opcode},
        symbol_table::{Symbol, SymbolScope, SymbolTable},
    },
    lexer::token::Token,
    object::{
        builtins::BuiltinFunction,
        {CompiledFunction, Object},
    },
    parser::ast::{
        BlockStatement, Conditional, Expression, FunctionLiteral, InfixOperator, Primitive,
        Program, Statement, WhileStatement,
    },
};

use num_traits::FromPrimitive;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct EmittedInstruction {
    opcode: Opcode,
    position: usize,
}

struct CompilerScope {
    instructions: Instructions,
    last_instruction: Option<EmittedInstruction>,
    previous_instruction: Option<EmittedInstruction>,
}

impl Default for CompilerScope {
    fn default() -> Self {
        Self::new()
    }
}

impl CompilerScope {
    pub fn new() -> Self {
        Self {
            instructions: Instructions::default(),
            last_instruction: None,
            previous_instruction: None,
        }
    }
}

pub struct Compiler {
    pub constants: Vec<Object>,

    pub symbol_table: SymbolTable,

    scopes: Vec<CompilerScope>,
    scope_index: usize,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        let main_scope = CompilerScope::default();
        let mut symbol_table = SymbolTable::new();
        for (i, builtin) in BuiltinFunction::get_builtins_names().iter().enumerate() {
            symbol_table.define_builtin(i, builtin.to_string());
        }

        Compiler {
            constants: vec![],

            symbol_table,

            scopes: vec![main_scope],
            scope_index: 0,
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
                // This step is extremely important. If it is not done then when shadowing variables
                // and using the previous value we get an error. Because we would have assigned
                // a new index to the symbol and the GetGlobal instruction would get a NULL
                // value instead of the previous value. (corresponds to issue #8)
                let symbol = match self.symbol_table.resolve(&s.name.value) {
                    Some(symbol) => match symbol.scope {
                        SymbolScope::Global => {
                            // A Local variable should never replace a global one
                            if self.symbol_table.has_outer() {
                                // This means that the symbol will
                                // be local and not global, and thus not
                                // replace the global one
                                self.symbol_table.define(s.name.value)
                            } else {
                                symbol
                            }
                        }
                        SymbolScope::Local => symbol,

                        // We only want to do in in the case of "normal" variable assignation.
                        // The special cases should not be touched, since the program should not
                        // have access to them, only the compiler/vm
                        _ => self.symbol_table.define(s.name.value),
                    },
                    None => self.symbol_table.define(s.name.value),
                };

                self.compile_expression(s.value)?;

                match symbol.scope {
                    SymbolScope::Global => {
                        self.emit(Opcode::SetGlobal, vec![symbol.index as i32]);
                    }
                    SymbolScope::Local => {
                        self.emit(Opcode::SetLocal, vec![symbol.index as i32]);
                    }
                    SymbolScope::Free => {
                        unreachable!(
                            "Free symbols should not be set, the compiler should panic before this"
                        )
                    }
                    SymbolScope::Builtin => {
                        unreachable!("Builtin symbols should not be set, the compiler should panic before this")
                    }
                    SymbolScope::Function => {
                        unreachable!("Function symbols should not be set, the compiler should panic before this")
                    }
                }
            }
            Statement::Return(r) => {
                self.compile_expression(r.return_value)?;
                self.emit(Opcode::ReturnValue, vec![]);
            }
            Statement::While(wh) => {
                self.compile_while_statement(wh)?;
            }
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
                    Some(symbol) => self.load_symbol(&symbol),
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

            Expression::HashMapLiteral(hasmap) => {
                let len = i32::from_usize(hasmap.pairs.len()).ok_or("Invalid hashmap length")?;
                for (key, value) in hasmap.pairs {
                    self.compile_expression(key)?;
                    self.compile_expression(value)?;
                }
                self.emit(Opcode::HashMap, vec![len * 2]);
            }
            Expression::IndexExpression(index) => {
                self.compile_expression(*index.left)?;
                self.compile_expression(*index.index)?;
                self.emit(Opcode::Index, vec![]);
            }
            Expression::FunctionLiteral(fun) => {
                self.compile_function_literal(fun)?;
            }
            Expression::FunctionCall(call) => {
                self.compile_expression(*call.function)?;

                let args_length =
                    i32::from_usize(call.arguments.len()).ok_or("Invalid argument length")?;

                for argument in call.arguments {
                    self.compile_expression(argument)?;
                }

                self.emit(Opcode::Call, vec![args_length]);
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
        if self.last_instruction_is(Opcode::Pop) {
            self.remove_last_instruction();
        }

        let jump_pos = self.emit(Opcode::Jump, vec![9999]); // We emit a dummy value for the jump offset
                                                            // and we will fix it later

        let after_consequence_pos = self.current_instructions().data.len();
        self.change_operand(jump_not_truthy_pos, after_consequence_pos as i32)?;

        if let Some(alternative) = conditional.alternative {
            self.compile_block_statement(alternative)?;
            if self.last_instruction_is(Opcode::Pop) {
                self.remove_last_instruction();
            }
        } else {
            self.emit(Opcode::Null, vec![]);
        }

        let after_alternative_pos = self.current_instructions().data.len();
        self.change_operand(jump_pos, after_alternative_pos as i32)?;

        Ok(())
    }

    fn compile_function_literal(&mut self, fun: FunctionLiteral) -> Result<(), String> {
        self.enter_scope();

        if let Some(name) = fun.name {
            self.symbol_table.define_function_name(name);
        }

        let num_parameters = fun.parameters.len();

        for param in fun.parameters {
            self.symbol_table.define(param.value);
        }

        self.compile_block_statement(fun.body)?;

        if self.last_instruction_is(Opcode::Pop) {
            self.replace_last_pop_with_return();
        }
        if !self.last_instruction_is(Opcode::ReturnValue) {
            self.emit(Opcode::Return, vec![]);
        }

        let free_symbols = self.symbol_table.free_symbols.clone();
        let free_symbols_len = free_symbols.len();

        let num_locals = self.symbol_table.num_definitions;
        let instructions = self.leave_scope().data;

        for symbol in free_symbols {
            // Te symbols must be loaded after the scope is left, but
            // we need to get them before leaving the scope.
            self.load_symbol(&symbol);
        }

        let compiled_function = Object::COMPILEDFUNCTION(CompiledFunction {
            instructions,
            num_locals,
            num_parameters,
        });

        let operands =
            i32::from_usize(self.add_constant(compiled_function)).ok_or("Invalid integer type")?;

        self.emit(Opcode::Closure, vec![operands, free_symbols_len as i32]);

        Ok(())
    }

    fn compile_while_statement(&mut self, wh: WhileStatement) -> Result<(), String> {
        let condition_pos = self.current_instructions().data.len();
        self.compile_expression(wh.condition)?;

        let jump_not_truthy_pos = self.emit(Opcode::JumpNotTruthy, vec![9999]); // We emit a dummy value for the jump offset
                                                                                // and we will fix it later
        self.compile_block_statement(wh.body)?;

        self.emit(Opcode::Jump, vec![condition_pos as i32]); // We emit a dummy value for the jump offset
                                                             // and we will fix it later

        let after_body_pos = self.current_instructions().data.len();
        self.change_operand(jump_not_truthy_pos, after_body_pos as i32)?;

        Ok(())
    }

    fn last_instruction_is(&self, opcode: Opcode) -> bool {
        match self.scopes[self.scope_index].last_instruction {
            Some(ref last) => last.opcode == opcode,
            None => false,
        }
    }

    fn remove_last_instruction(&mut self) {
        if let Some(last) = self.scopes[self.scope_index].last_instruction.clone() {
            let previous = self.scopes[self.scope_index].previous_instruction.clone();

            let old = self.current_instructions().data;
            let new = old[..last.position].to_vec();

            self.scopes[self.scope_index].instructions.data = new;
            self.scopes[self.scope_index].last_instruction = previous;
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
        let pos_new_instruction = self.current_instructions().data.len();
        self.scopes[self.scope_index]
            .instructions
            .append(instruction);
        pos_new_instruction
    }

    fn set_last_instruction(&mut self, opcode: Opcode, pos: usize) {
        let previous = self.scopes[self.scope_index].last_instruction.clone();
        let last = EmittedInstruction {
            opcode,
            position: pos,
        };
        self.scopes[self.scope_index].previous_instruction = previous;
        self.scopes[self.scope_index].last_instruction = Some(last);
    }

    fn change_operand(&mut self, pos: usize, operand: i32) -> Result<(), String> {
        let op = Opcode::from_u8(self.current_instructions().data[pos]).ok_or(format!(
            "Unknown opcode: {opcode}",
            opcode = self.current_instructions().data[pos]
        ))?;
        let new_instruction = op.make(vec![operand]);
        self.replace_instruction(pos, &new_instruction);
        Ok(())
    }

    fn replace_instruction(&mut self, pos: usize, new_instruction: &Instructions) {
        let ins = &mut self.scopes[self.scope_index].instructions;
        for (i, instruction) in new_instruction.data.iter().enumerate() {
            ins.data[pos + i] = *instruction;
        }
    }

    fn current_instructions(&self) -> Instructions {
        self.scopes[self.scope_index].instructions.clone()
    }

    fn enter_scope(&mut self) {
        let scope = CompilerScope::default();
        self.symbol_table =
            SymbolTable::new_enclosed(Rc::new(RefCell::new(self.symbol_table.clone())));
        self.scopes.push(scope);
        self.scope_index += 1;
    }

    fn leave_scope(&mut self) -> Instructions {
        let instructions = self.current_instructions();

        self.symbol_table = self
            .symbol_table
            .outer
            .clone()
            .unwrap()
            .as_ref()
            .clone()
            .into_inner();

        self.scopes.pop();
        self.scope_index -= 1;

        instructions
    }

    fn replace_last_pop_with_return(&mut self) {
        let last_pos = self.scopes[self.scope_index]
            .last_instruction
            .as_ref()
            .unwrap()
            .position;
        self.replace_instruction(last_pos, &Opcode::ReturnValue.make(vec![]));
        self.scopes[self.scope_index]
            .last_instruction
            .as_mut()
            .unwrap()
            .opcode = Opcode::ReturnValue;
    }

    fn load_symbol(&mut self, symbol: &Symbol) {
        let opcode = match symbol.scope {
            SymbolScope::Global => Opcode::GetGlobal,
            SymbolScope::Local => Opcode::GetLocal,
            SymbolScope::Builtin => Opcode::GetBuiltin,
            SymbolScope::Free => Opcode::GetFree,
            SymbolScope::Function => Opcode::CurrentClosure,
        };

        self.emit(opcode, vec![symbol.index as i32]);
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode::new(self.current_instructions(), self.constants.clone())
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

    use super::*;

    #[test]
    fn test_compiler_scopes() {
        let mut compiler = Compiler::new();

        assert_eq!(compiler.scope_index, 0);

        let global_symbol_table = compiler.symbol_table.clone();

        compiler.emit(Opcode::Mul, vec![]);

        compiler.enter_scope();
        assert_eq!(compiler.scope_index, 1);

        compiler.emit(Opcode::Sub, vec![]);
        assert_eq!(
            compiler.scopes[compiler.scope_index]
                .instructions
                .data
                .len(),
            1
        );

        let last = compiler.scopes[compiler.scope_index]
            .last_instruction
            .clone()
            .unwrap();
        assert_eq!(last.opcode, Opcode::Sub);

        assert_eq!(
            compiler.symbol_table.outer,
            Some(Rc::new(RefCell::new(global_symbol_table.clone()))),
            "Compiler did not enclose symbol table when entering new scope"
        );

        compiler.leave_scope();
        assert_eq!(compiler.scope_index, 0);

        assert_eq!(
            compiler.symbol_table,
            (global_symbol_table),
            "Compiler did not restore global symbol table after leaving scope"
        );

        compiler.emit(Opcode::Add, vec![]);
        assert_eq!(
            compiler.scopes[compiler.scope_index]
                .instructions
                .data
                .len(),
            2
        );

        let last = compiler.scopes[compiler.scope_index]
            .last_instruction
            .clone()
            .unwrap();
        assert_eq!(last.opcode, Opcode::Add);

        let previous = compiler.scopes[compiler.scope_index]
            .previous_instruction
            .clone()
            .unwrap();
        assert_eq!(previous.opcode, Opcode::Mul);
    }
}
