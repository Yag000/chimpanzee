use std::rc::Rc;

use num_traits::FromPrimitive;

use crate::{
    code::{read_u16, Instructions, Opcode},
    compiler::Bytecode,
    evaluator::object::Object,
};

const STACK_SIZE: usize = 2048;

pub struct VM {
    constants: Vec<Rc<Object>>,
    instructions: Instructions,

    stack: Vec<Rc<Object>>,
    sp: usize, // stack pointer. Always point to the next value. Top of the stack is stack[sp -1]
}

impl VM {
    fn new(bytecode: Bytecode) -> Self {
        Self {
            instructions: bytecode.instructions,
            constants: bytecode.constants.into_iter().map(Rc::new).collect(), // TODO: Improve this

            stack: Vec::with_capacity(STACK_SIZE),
            sp: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut ip = 0;
        while ip < self.instructions.data.len() {
            let op = Opcode::from_u8(self.instructions.data[ip])
                .ok_or(format!("Unknown opcode {}", self.instructions.data[ip]))?;
            match op {
                Opcode::Opconstant => {
                    let const_index = read_u16(&self.instructions.data[ip + 1..]);
                    ip += 2;
                    self.push(self.constants[const_index as usize].clone())?;
                }
            }
            ip += 1;
        }
        Ok(())
    }

    fn push(&mut self, obj: Rc<Object>) -> Result<(), String> {
        if self.sp >= STACK_SIZE {
            Err("Stack overflow :(, you gotta fix this".to_string())
        } else {
            self.stack.push(obj);
            self.sp += 1;
            Ok(())
        }
    }

    fn stack_top(&self) -> Option<Rc<Object>> {
        self.stack.get(self.sp - 1).cloned()
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        compiler::{
            tests::{parse, test_constants},
            Compiler,
        },
        evaluator::object::Object,
    };

    use super::VM;

    struct VmTestCase {
        input: String,
        expected: Object,
    }

    fn run_vm_tests(tests: Vec<VmTestCase>) {
        for test in tests {
            let program = parse(&test.input);
            let mut compiler = Compiler::new();
            compiler.compile(program).unwrap();
            let bytecode = compiler.bytecode();

            let mut vm = VM::new(bytecode);
            vm.run().unwrap();
            let got = vm.stack_top().unwrap();
            test_constants(&vec![test.expected], &vec![got]);
        }
    }

    #[test]
    fn test_integer_arithmetic() {
        let tests = vec![
            VmTestCase {
                input: "1".to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: "2".to_string(),
                expected: Object::INTEGER(2),
            },
            VmTestCase {
                input: "1 + 2".to_string(),
                expected: Object::INTEGER(2),
            },
        ];
        run_vm_tests(tests);
    }
}
