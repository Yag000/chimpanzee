use compiler::{
    code::{read_u16, Instructions, Opcode},
    compiler::Bytecode,
};
use interpreter::object::Object;
use num_traits::FromPrimitive;
use std::rc::Rc;

const STACK_SIZE: usize = 2048;
pub const GLOBALS_SIZE: usize = 65536;

pub const NULL: Object = Object::NULL;
const TRUE: Object = Object::BOOLEAN(true);
const FALSE: Object = Object::BOOLEAN(false);

pub struct VM {
    constants: Vec<Rc<Object>>,
    instructions: Instructions,

    stack: Vec<Rc<Object>>,
    sp: usize, // stack pointer. Always point to the next value. Top of the stack is stack[sp -1]

    pub globals: Vec<Rc<Object>>,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            instructions: bytecode.instructions,
            constants: bytecode.constants.into_iter().map(Rc::new).collect(), // TODO: Improve this

            sp: 0,
            stack: {
                let mut v = Vec::with_capacity(STACK_SIZE);
                (0..STACK_SIZE).for_each(|_| v.push(Rc::new(NULL)));
                v
            },

            globals: {
                let mut v = Vec::with_capacity(GLOBALS_SIZE);
                (0..GLOBALS_SIZE).for_each(|_| v.push(Rc::new(NULL)));
                v
            },
        }
    }

    pub fn new_with_global_store(bytecode: Bytecode, globals: Vec<Rc<Object>>) -> Self {
        let mut vm = Self::new(bytecode);
        vm.globals = globals;
        vm
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
                Opcode::Add
                | Opcode::Sub
                | Opcode::Mul
                | Opcode::Div
                | Opcode::Or
                | Opcode::And => {
                    self.execute_binary_operation(op)?;
                }
                Opcode::Equal
                | Opcode::NotEqual
                | Opcode::GreaterThan
                | Opcode::GreaterEqualThan => {
                    self.execute_comparison(op)?;
                }
                Opcode::Pop => {
                    self.pop();
                }
                Opcode::True => {
                    self.push(Rc::new(TRUE))?;
                }
                Opcode::False => {
                    self.push(Rc::new(FALSE))?;
                }
                Opcode::Bang => {
                    self.execute_bang_operation()?;
                }
                Opcode::Minus => {
                    self.execute_minus_operation()?;
                }
                Opcode::Jump => {
                    let pos = read_u16(&self.instructions.data[ip + 1..]) as usize;
                    ip = pos - 1;
                }
                Opcode::JumpNotTruthy => {
                    let pos = read_u16(&self.instructions.data[ip + 1..]) as usize;
                    ip += 2;
                    let condition = self.pop().ok_or("Stack underflow".to_string())?;
                    if !self.is_truthy(&condition) {
                        ip = pos - 1;
                    }
                }
                Opcode::Null => {
                    self.push(Rc::new(NULL))?;
                }
                Opcode::SetGlobal => {
                    let global_index = read_u16(&self.instructions.data[ip + 1..]) as usize;
                    ip += 2;
                    let value = self.pop().ok_or("Stack underflow".to_string())?;
                    self.globals[global_index] = value;
                }

                Opcode::GetGlobal => {
                    let global_index = read_u16(&self.instructions.data[ip + 1..]) as usize;
                    ip += 2;
                    self.push(self.globals[global_index].clone())?;
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
                self.execute_bianary_integer_operation(&left, &right, op)?;
            }
            (Object::BOOLEAN(left), Object::BOOLEAN(right)) => {
                let result = match op {
                    Opcode::Or => *left || *right,
                    Opcode::And => *left && *right,
                    _ => {
                        return Err("Unsupported types for binary operation".to_string());
                    }
                };

                if result {
                    self.push(Rc::new(TRUE))?;
                } else {
                    self.push(Rc::new(FALSE))?;
                }
            }
            (Object::STRING(s1), Object::STRING(s2)) => {
                let result = match op {
                    Opcode::Add => s1.to_string() + s2,
                    _ => {
                        return Err("Unsupported types for binary operation".to_string());
                    }
                };

                self.push(Rc::new(Object::STRING(result)))?;
            }
            _ => return Err("Unsupported types for binary operation".to_string()),
        }
        Ok(())
    }

    fn execute_bianary_integer_operation(
        &mut self,
        left: &Rc<Object>,
        right: &Rc<Object>,
        op: Opcode,
    ) -> Result<(), String> {
        let left = self
            .cast_to_integer(left)
            .ok_or("Error: Not an integer".to_string())?;
        let right = self
            .cast_to_integer(right)
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

    fn execute_comparison(&mut self, op: Opcode) -> Result<(), String> {
        let right = self.pop().ok_or("Stack underflow".to_string())?;
        let left = self.pop().ok_or("Stack underflow".to_string())?;

        match (&*left, &*right) {
            (Object::INTEGER(_), Object::INTEGER(_)) => {
                self.execute_integer_comparison(&left, &right, op)?;
            }
            (Object::BOOLEAN(_), Object::BOOLEAN(_)) => match op {
                Opcode::Equal => {
                    self.push(self.native_boolean_to_boolean_object(left == right))?;
                }
                Opcode::NotEqual => {
                    self.push(self.native_boolean_to_boolean_object(left != right))?;
                }
                _ => Err("Unsupported types for comparison".to_string())?,
            },
            _ => Err("Unsupported types for comparison".to_string())?,
        }
        Ok(())
    }

    fn execute_integer_comparison(
        &mut self,
        left: &Rc<Object>,
        right: &Rc<Object>,
        op: Opcode,
    ) -> Result<(), String> {
        let left = self
            .cast_to_integer(left)
            .ok_or("Error: Not an integer".to_string())?;
        let right = self
            .cast_to_integer(right)
            .ok_or("Error: Not an integer".to_string())?;

        let result = match op {
            Opcode::Equal => left == right,
            Opcode::NotEqual => left != right,
            Opcode::GreaterThan => left > right,
            Opcode::GreaterEqualThan => left >= right,
            _ => unreachable!(),
        };

        if result {
            self.push(Rc::new(TRUE))?;
        } else {
            self.push(Rc::new(FALSE))?;
        }
        Ok(())
    }

    fn execute_bang_operation(&mut self) -> Result<(), String> {
        let operand = self.pop().ok_or("Stack underflow".to_string())?;
        let value = self.native_boolean_to_boolean_object(!self.is_truthy(&operand));
        self.push(value)?;
        Ok(())
    }

    fn execute_minus_operation(&mut self) -> Result<(), String> {
        let operand = self.pop().ok_or("Stack underflow".to_string())?;

        match &*operand {
            Object::INTEGER(i) => {
                self.push(Rc::new(Object::INTEGER(-i)))?;
            }
            _ => {
                return Err("Unsupported type for minus operation".to_string());
            }
        }
        Ok(())
    }

    fn native_boolean_to_boolean_object(&self, input: bool) -> Rc<Object> {
        if input {
            Rc::new(TRUE)
        } else {
            Rc::new(FALSE)
        }
    }

    fn is_truthy(&self, obj: &Rc<Object>) -> bool {
        match &**obj {
            Object::NULL => false,
            Object::BOOLEAN(b) => *b,
            _ => true,
        }
    }

    fn push(&mut self, obj: Rc<Object>) -> Result<(), String> {
        if self.sp >= STACK_SIZE {
            Err("Stack overflow :(, you gotta fix this".to_string())
        } else {
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

    use compiler::{
        compiler::Compiler,
        test_utils::{check_constants, parse},
    };
    use interpreter::object::Object;

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
            check_constants(&vec![test.expected], &vec![got]);
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
            VmTestCase {
                input: "-1".to_string(),
                expected: Object::INTEGER(-1),
            },
            VmTestCase {
                input: "-10".to_string(),
                expected: Object::INTEGER(-10),
            },
            VmTestCase {
                input: "-50 + 100 + -50".to_string(),
                expected: Object::INTEGER(0),
            },
            VmTestCase {
                input: "(5 + 10 * 2 + 15 / 3) * 2 + -10".to_string(),
                expected: Object::INTEGER(50),
            },
        ];
        run_vm_tests(tests);
    }

    #[test]
    fn test_boolean_logic() {
        let tests = vec![
            VmTestCase {
                input: "true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "1 < 2".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "1 <= 2".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "2 <= 2".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "1 > 2".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "1 >= 2".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "2 >= 2".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "1 == 2".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "1 != 2".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "true == true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "false == false".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "true == false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "true && true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "true && false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "false && true".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "false && false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "true || true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "true || false".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "false || true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "false || false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "!true".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "!false".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "!5".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "!!true".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "!!false".to_string(),
                expected: Object::BOOLEAN(false),
            },
            VmTestCase {
                input: "!!5".to_string(),
                expected: Object::BOOLEAN(true),
            },
            VmTestCase {
                input: "!(if (false) { 5 })".to_string(),
                expected: Object::BOOLEAN(true),
            },
        ];
        run_vm_tests(tests);
    }
    #[test]
    fn test_conditionals() {
        let tests = vec![
            VmTestCase {
                input: "if (true) { 10 }".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "if (true) { 10 } else { 20 }".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "if (false) { 10 } else { 20 } ".to_string(),
                expected: Object::INTEGER(20),
            },
            VmTestCase {
                input: "if (1) { 10 }".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "if (1 < 2) { 10 }".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "if (1 < 2) { 10 } else { 20 }".to_string(),
                expected: Object::INTEGER(10),
            },
            VmTestCase {
                input: "if (1 > 2) { 10 } else { 20 }".to_string(),
                expected: Object::INTEGER(20),
            },
            VmTestCase {
                input: "if (1 > 2) { 10 }".to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: "if (false) { 10 }".to_string(),
                expected: Object::NULL,
            },
            VmTestCase {
                input: "if ((if (false) { 10 })) { 10 } else { 20 }".to_string(),
                expected: Object::INTEGER(20),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_global_let_statements() {
        let tests = vec![
            VmTestCase {
                input: "let one = 1; one".to_string(),
                expected: Object::INTEGER(1),
            },
            VmTestCase {
                input: "let one = 1; let two = 2; one + two".to_string(),
                expected: Object::INTEGER(3),
            },
            VmTestCase {
                input: "let one = 1; let two = one + one; one + two".to_string(),
                expected: Object::INTEGER(3),
            },
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_string_expressions() {
        let tests = vec![
            VmTestCase {
                input: "\"monkey\"".to_string(),
                expected: Object::STRING("monkey".to_string()),
            },
            VmTestCase {
                input: "\"mon\" + \"key\"".to_string(),
                expected: Object::STRING("monkey".to_string()),
            },
            VmTestCase {
                input: "\"mon\" + \"key\" + \"banana\"".to_string(),
                expected: Object::STRING("monkeybanana".to_string()),
            },
        ];

        run_vm_tests(tests);
    }
}
