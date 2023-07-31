use compiler::{
    code::{read_u16, Opcode},
    compiler::Bytecode,
};
use num_traits::FromPrimitive;
use object::{
    builtins::BuiltinFunction,
    object::{Closure, CompiledFunction, Object, FALSE, NULL, TRUE},
};
use std::{collections::HashMap, rc::Rc};

const STACK_SIZE: usize = 2048;
const MAX_FRAMES: usize = 1024;
pub const GLOBALS_SIZE: usize = 65536;

#[derive(Debug)]
struct Frame {
    function: Closure,
    ip: i32,
    base_pointer: usize,
}

impl Frame {
    fn new(function: Closure, base_pointer: usize) -> Self {
        Self {
            function,
            ip: -1,
            base_pointer,
        }
    }

    fn get_instructions(&self) -> &Vec<u8> {
        &self.function.function.instructions
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
            num_locals: 0,
            num_parameters: 0,
        };
        let main_closure = Closure::new(main_function);
        let main_frame = Frame::new(main_closure, 0);
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

    #[allow(clippy::too_many_lines)]
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
                    let pos = i32::from(read_u16(&ins[ip + 1..]));
                    self.current_frame().ip = pos - 1;
                }
                Opcode::JumpNotTruthy => {
                    let pos = i32::from(read_u16(&ins[ip + 1..]));
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
                Opcode::SetLocal => {
                    let local_index = ins[ip + 1] as usize;
                    self.current_frame().ip += 1;
                    let value = self.pop()?;
                    let base_pointer = self.current_frame().base_pointer;
                    self.stack[base_pointer + local_index] = value;
                }
                Opcode::GetLocal => {
                    let local_index = ins[ip + 1] as usize;
                    self.current_frame().ip += 1;
                    let base_pointer = self.current_frame().base_pointer;
                    let value = Rc::clone(&self.stack[base_pointer + local_index]);
                    self.push(value)?;
                }

                Opcode::GetBuiltin => {
                    let builtin_index = ins[ip + 1] as usize;
                    self.current_frame().ip += 1;

                    let def = BuiltinFunction::get_builtin_by_id(builtin_index)
                        .ok_or(format!("Unknown builtin function id {builtin_index}"))?;

                    self.push(Rc::new(def))?;
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
                    let num_args = ins[ip + 1] as usize;
                    self.current_frame().ip += 1;

                    self.execute_call(num_args)?;
                }
                Opcode::ReturnValue => {
                    let return_value = self.pop()?;

                    match self.pop_frame() {
                        Some(frame) => self.sp = frame.base_pointer - 1,
                        None => Err("There was no frame")?,
                    }

                    self.push(return_value)?;
                }
                Opcode::Return => {
                    match self.pop_frame() {
                        Some(frame) => self.sp = frame.base_pointer - 1,
                        None => Err("There was no frame")?,
                    }

                    self.push(Rc::new(NULL))?;
                }
                Opcode::Closure => {
                    let const_index = read_u16(&ins[ip + 1..]) as usize;
                    let _free = ins[ip + 3] as usize;

                    self.current_frame().ip += 3;

                    self.push_closure(const_index)?;
                }
                Opcode::GetFree => {
                    unimplemented!();
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

    fn execute_call(&mut self, num_args: usize) -> Result<(), String> {
        let callee = self
            .stack
            .get(self.sp - 1 - num_args)
            .ok_or("Stack underflow")?;

        match callee.as_ref().clone() {
            Object::CLOSURE(func) => self.call_closure(func, num_args),
            Object::BUILTIN(func) => self.call_builtin_function(&func, num_args),
            _ => Err("Calling non-function".to_string()),
        }
    }

    fn call_closure(&mut self, func: Closure, num_args: usize) -> Result<(), String> {
        if num_args != func.function.num_parameters {
            return Err(format!(
                "Wrong number of arguments: want={}, got={}",
                func.function.num_parameters, num_args
            ));
        }

        let num_locals = func.function.num_locals;
        let frame = Frame::new(func, self.sp - num_args);
        self.sp = frame.base_pointer + num_locals;
        self.push_frame(frame);
        Ok(())
    }

    fn call_builtin_function(
        &mut self,
        callee: &BuiltinFunction,
        num_args: usize,
    ) -> Result<(), String> {
        let mut args: Vec<Object> = Vec::new();
        for _ in 0..num_args {
            args.push(self.pop()?.as_ref().clone());
        }
        args.reverse();

        let result = callee.call(args);

        self.sp -= 1;
        self.push(Rc::new(result))?;
        Ok(())
    }

    fn push_closure(&mut self, const_index: usize) -> Result<(), String> {
        match (*self.constants[const_index]).clone() {
            Object::COMPILEDFUNCTION(func) => {
                let closure = Closure::new(func);
                self.push(Rc::new(Object::CLOSURE(closure)))
            }
            x => Err(format!["Function expected, got {}", x.get_type()]),
        }
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
