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
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            instructions: bytecode.instructions,
            constants: bytecode.constants.into_iter().map(Rc::new).collect(), // TODO: Improve this

            sp: 0,
            stack: {
                let mut v = Vec::with_capacity(STACK_SIZE);
                (0..STACK_SIZE).for_each(|_| v.push(Rc::new(Object::NULL)));
                v
            },
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut ip = 0;
        while ip < self.instructions.data.len() {
            let op = Opcode::from_u8(self.instructions.data[ip])
                .ok_or(format!("Unknown opcode {}", self.instructions.data[ip]))?;
            match op {
                Opcode::Constant => {
                    let const_index = read_u16(&self.instructions.data[ip + 1..]);
                    ip += 2;
                    self.push(self.constants[const_index as usize].clone())?;
                }
                Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div => {
                    self.execute_binary_operation(op)?;
                }
                Opcode::Pop => {
                    self.pop();
                }
            }
            ip += 1;
        }
        Ok(())
    }

    fn execute_binary_operation(&mut self, op: Opcode) -> Result<(), String> {
        let right = self.pop().ok_or("Stack underflow".to_string())?;
        let left = self.pop().ok_or("Stack underflow".to_string())?;

        match (&*left, &*right) {
            (Object::INTEGER(_), Object::INTEGER(_)) => {
                self.execute_bianary_integer_operation(left, right, op)
            }
            _ => Err("Unsupported types for binary operation".to_string()),
        }
    }

    fn execute_bianary_integer_operation(
        &mut self,
        left: Rc<Object>,
        right: Rc<Object>,
        op: Opcode,
    ) -> Result<(), String> {
        let left = self
            .cast_to_integer(&left)
            .ok_or("Error: Not an integer".to_string())?;
        let right = self
            .cast_to_integer(&right)
            .ok_or("Error: Not an integer".to_string())?;

        let result = match op {
            Opcode::Add => left + right,
            Opcode::Sub => left - right,
            Opcode::Mul => left * right,
            Opcode::Div => left / right,
            _ => unreachable!(),
        };

        self.push(Rc::new(Object::INTEGER(result)))?;
        Ok(())
    }

    fn push(&mut self, obj: Rc<Object>) -> Result<(), String> {
        if self.sp >= STACK_SIZE {
            Err("Stack overflow :(, you gotta fix this".to_string())
        } else {
            println!("Pushing {:?}, at pos {}", obj, self.sp);
            self.stack[self.sp] = obj;
            self.sp += 1;
            Ok(())
        }
    }

    fn pop(&mut self) -> Option<Rc<Object>> {
        if self.sp == 0 {
            None
        } else {
            self.sp -= 1;
            self.stack.get(self.sp).cloned()
        }
    }

    pub fn stack_top(&self) -> Option<Rc<Object>> {
        self.stack.get(self.sp - 1).cloned()
    }

    fn cast_to_integer(&self, obj: &Rc<Object>) -> Option<i64> {
        match **obj {
            Object::INTEGER(i) => Some(i),
            _ => None,
        }
    }

    pub fn last_popped_stack_element(&self) -> Option<Rc<Object>> {
        self.stack.get(self.sp).cloned()
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
            println!("Running test: {}", test.input);
            let program = parse(&test.input);
            let mut compiler = Compiler::new();
            compiler.compile(program).unwrap();
            let bytecode = compiler.bytecode();

            let mut vm = VM::new(bytecode);
            vm.run().unwrap();
            let got = vm.last_popped_stack_element().unwrap();
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
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: "1 - 2".to_string(),
                expected: Object::INTEGER(-1),
            },
            VmTestCase {
                input: "1 * 2".to_string(),
                expected: Object::INTEGER(2),
            },
            VmTestCase {
                input: "4 / 2".to_string(),
                expected: Object::INTEGER(2),
            },
            VmTestCase {
                input: "50 / 2 * 2 + 10 - 5".to_string(),
                expected: Object::INTEGER(55),
            },
            VmTestCase {
                input: "5 + 5 + 5 + 5 - 10".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "2 * 2 * 2 * 2 * 2".to_string(),
                expected: Object::INTEGER(32),
            },
            VmTestCase {
                input: "5 * 2 + 10".to_string(),
                expected: Object::INTEGER(20),
            },
        ];
        run_vm_tests(tests);
    }
}
