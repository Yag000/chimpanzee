use compiler::{
    code::{read_u16, Opcode},
    compiler::Bytecode,
};
use num_traits::FromPrimitive;
use object::object::{CompiledFunction, Object, FALSE, NULL, TRUE};
use std::{collections::HashMap, rc::Rc};

const STACK_SIZE: usize = 2048;
const MAX_FRAMES: usize = 1024;
pub const GLOBALS_SIZE: usize = 65536;

#[derive(Debug)]
struct Frame {
    function: CompiledFunction,
    ip: i32,
}

impl Frame {
    fn new(function: CompiledFunction) -> Self {
        Self { function, ip: -1 }
    }

    fn get_instructions(&self) -> &Vec<u8> {
        &self.function.instructions
    }
}

pub struct VM {
    constants: Vec<Rc<Object>>,

    stack: Vec<Rc<Object>>,
    sp: usize, // stack pointer. Always point to the next value. Top of the stack is stack[sp -1]

    pub globals: Vec<Rc<Object>>,

    frames: Vec<Frame>,
    frames_index: usize,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        let main_function = CompiledFunction {
            instructions: bytecode.instructions.data,
        };
        let main_frame = Frame::new(main_function);
        let mut frames = Vec::with_capacity(MAX_FRAMES);
        frames.push(main_frame);
        Self {
            constants: bytecode.constants.into_iter().map(Rc::new).collect(),

            sp: 0,

            // TODO: Improve this
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

            frames,
            frames_index: 1,
        }
    }

    pub fn new_with_global_store(bytecode: Bytecode, globals: Vec<Rc<Object>>) -> Self {
        let mut vm = Self::new(bytecode);
        vm.globals = globals;
        vm
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.current_frame().ip < self.current_frame().get_instructions().len() as i32 - 1 {
            self.current_frame().ip += 1;
            let ip = self.current_frame().ip as usize;
            let ins = self.current_frame().get_instructions();
            let op = Opcode::from_u8(ins[ip]).ok_or(format!("Unknown opcode {}", ins[ip]))?;
            match op {
                Opcode::Constant => {
                    let const_index = read_u16(&ins[ip + 1..]);
                    self.current_frame().ip += 2;
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
                    self.pop()?;
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
                    let pos = read_u16(&ins[ip + 1..]) as i32;
                    self.current_frame().ip = pos - 1;
                }
                Opcode::JumpNotTruthy => {
                    let pos = read_u16(&ins[ip + 1..]) as i32;
                    self.current_frame().ip += 2;
                    let condition = self.pop()?;
                    if !self.is_truthy(&condition) {
                        self.current_frame().ip = pos - 1;
                    }
                }
                Opcode::Null => {
                    self.push(Rc::new(NULL))?;
                }
                Opcode::SetGlobal => {
                    let global_index = read_u16(&ins[ip + 1..]) as usize;
                    self.current_frame().ip += 2;
                    let value = self.pop()?;
                    self.globals[global_index] = value;
                }

                Opcode::GetGlobal => {
                    let global_index = read_u16(&ins[ip + 1..]) as usize;
                    self.current_frame().ip += 2;
                    self.push(self.globals[global_index].clone())?;
                }

                Opcode::Array => {
                    let num_elements = read_u16(&ins[ip + 1..]) as usize;
                    self.current_frame().ip += 2;
                    let array = self.build_array(self.sp - num_elements, self.sp)?;
                    self.sp -= num_elements;
                    self.push(array)?;
                }
                Opcode::HashMap => {
                    let num_elements = read_u16(&ins[ip + 1..]) as usize;
                    self.current_frame().ip += 2;
                    let hashmap = self.build_hashmap(self.sp - num_elements, self.sp)?;
                    self.sp -= num_elements;
                    self.push(hashmap)?;
                }
                Opcode::Index => {
                    let index = self.pop()?;
                    let left = self.pop()?;
                    self.execute_index_expression(&left, &index)?;
                }
                Opcode::Call => {
                    let func = self
                        .stack
                        .get(self.sp - 1)
                        .ok_or("Stack underflow")?
                        .as_ref();
                    if let Object::COMPILEDFUNCTION(compiled) = func {
                        let frame = Frame::new(compiled.clone());
                        self.push_frame(frame);
                    } else {
                        Err("Calling non-function")?;
                    }
                }
                Opcode::ReturnValue => {
                    let return_value = self.pop()?;

                    self.pop_frame();
                    self.pop()?;

                    self.push(return_value)?;
                }
                Opcode::Return => {
                    self.pop_frame();
                    self.pop()?;

                    self.push(Rc::new(NULL))?;
                }
            }
        }
        Ok(())
    }

    fn execute_binary_operation(&mut self, op: Opcode) -> Result<(), String> {
        let right = self.pop()?;
        let left = self.pop()?;

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
        let left = self.cast_to_integer(left)?;
        let right = self.cast_to_integer(right)?;

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
        let right = self.pop()?;
        let left = self.pop()?;

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
        let left = self.cast_to_integer(left)?;
        let right = self.cast_to_integer(right)?;

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
        let operand = self.pop()?;
        let value = self.native_boolean_to_boolean_object(!self.is_truthy(&operand));
        self.push(value)?;
        Ok(())
    }

    fn execute_minus_operation(&mut self) -> Result<(), String> {
        let operand = self.pop()?;

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

    fn build_array(&self, start_index: usize, end_index: usize) -> Result<Rc<Object>, String> {
        let mut elements: Vec<Object> = Vec::new();
        for i in start_index..end_index {
            elements
                .push((**(self.stack.get(i).ok_or("Unable to get element".to_string()))?).clone());
        }
        Ok(Rc::new(Object::ARRAY(elements)))
    }

    fn build_hashmap(&self, start_index: usize, end_index: usize) -> Result<Rc<Object>, String> {
        let mut elements: HashMap<Object, Object> = HashMap::new();
        for i in (start_index..end_index).step_by(2) {
            let key = (**(self.stack.get(i).ok_or("Unable to get element".to_string()))?).clone();
            let value = (**(self
                .stack
                .get(i + 1)
                .ok_or("Unable to get element".to_string()))?)
            .clone();
            if !Object::is_hashable(&key) {
                return Ok(Rc::new(Object::ERROR(format!(
                    "Unusable as hashmap key: {key:?}"
                ))));
            }
            elements.insert(key, value);
        }
        Ok(Rc::new(Object::HASHMAP(elements)))
    }

    fn execute_index_expression(
        &mut self,
        left: &Rc<Object>,
        index: &Rc<Object>,
    ) -> Result<(), String> {
        match (&**left, &**index) {
            (Object::ARRAY(elements), Object::INTEGER(i)) => {
                if *i < 0 || *i >= elements.len() as i64 {
                    self.push(Rc::new(Object::NULL))?;
                } else {
                    let result = elements
                        .get(*i as usize)
                        .ok_or("Index out of bounds".to_string())?;
                    self.push(Rc::new(result.clone()))?;
                }
            }
            (Object::HASHMAP(elements), _) => {
                if !Object::is_hashable(index) {
                    return Err("Unusable as hashmap key".to_string());
                }
                match elements.get(index) {
                    Some(value) => {
                        self.push(Rc::new(value.clone()))?;
                    }
                    None => {
                        self.push(Rc::new(Object::NULL))?;
                    }
                }
            }

            _ => {
                return Err("Unsupported types for index operation".to_string());
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

    fn pop(&mut self) -> Result<Rc<Object>, String> {
        if self.sp == 0 {
            Err("Stack underflow".to_string())
        } else {
            self.sp -= 1;
            self.stack
                .get(self.sp)
                .ok_or("Stack underflow".to_string())
                .cloned()
        }
    }

    pub fn stack_top(&self) -> Option<Rc<Object>> {
        self.stack.get(self.sp - 1).cloned()
    }

    fn cast_to_integer(&self, obj: &Rc<Object>) -> Result<i64, String> {
        match **obj {
            Object::INTEGER(i) => Ok(i),
            _ => Err("Unable to cast to integer".to_string()),
        }
    }

    pub fn last_popped_stack_element(&self) -> Result<Rc<Object>, String> {
        self.stack
            .get(self.sp)
            .ok_or("Stack underflow".to_string())
            .cloned()
    }

    fn current_frame(&mut self) -> &mut Frame {
        &mut self.frames[self.frames_index - 1]
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
        self.frames_index += 1;
    }

    fn pop_frame(&mut self) -> Option<Frame> {
        self.frames_index -= 1;
        self.frames.pop()
    }
}
