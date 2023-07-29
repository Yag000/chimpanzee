use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Instructions {
    pub data: Vec<u8>,
}

impl Default for Instructions {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        while i < self.data.len() {
            let op = Opcode::from_u8(self.data[i])
                .unwrap_or_else(|| panic!("ERROR: Unknown opcode: {}", self.data[i]));
            let widths = op.lookup_widths();
            let (operands, read) = Opcode::read_operands(&widths, &self.data[i + 1..]);
            writeln!(
                f,
                "{:04} {}",
                i,
                self.format_instruction(op, &widths, &operands)
            )?;
            i += 1 + read as usize;
        }
        Ok(())
    }
}

impl Instructions {
    pub fn new(data: Vec<u8>) -> Instructions {
        Instructions { data }
    }

    pub fn format_instruction(
        &self,
        operand: Opcode,
        widths: &Vec<u32>,
        operands: &Vec<i32>,
    ) -> String {
        let operand_count = widths.len();
        if operands.len() != operand_count {
            return format!(
                "ERROR: operand len {} does not match defined {}",
                operands.len(),
                operand_count
            );
        }

        match operand_count {
            1 => format!("{} {}", operand, operands[0]),
            0 => format!("{operand}"),
            _ => format!("Unhandeled operand_count for {operand}"),
        }
    }

    pub fn append(&mut self, mut new_instructions: Instructions) {
        self.data.append(&mut new_instructions.data);
    }
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive, Clone, Copy)]
pub enum Opcode {
    // Constants
    Constant,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Boolean
    True,
    False,
    GreaterThan,
    GreaterEqualThan,
    Equal,
    NotEqual,
    Or,
    And,

    // Prefix operators
    Minus,
    Bang,

    // Jump
    JumpNotTruthy,
    Jump,

    // Null
    Null,

    // Variable assignment
    SetGlobal,
    GetGlobal,

    SetLocal,
    GetLocal,

    // Custom types
    Array,
    HashMap,
    Index,

    // Functions
    Call,
    ReturnValue,
    Return,
    GetBuiltin,

    // Stack
    Pop,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Opcode::Constant => "OpConstant",
            Opcode::Add => "OpAdd",
            Opcode::Sub => "OpSub",
            Opcode::Mul => "OpMul",
            Opcode::Div => "OpDiv",
            Opcode::True => "OpTrue",
            Opcode::False => "OpFalse",
            Opcode::GreaterThan => "OpGreaterThan",
            Opcode::GreaterEqualThan => "OpGreaterEqualThan",
            Opcode::Equal => "OpEqual",
            Opcode::NotEqual => "OpNotEqual",
            Opcode::Or => "OpOr",
            Opcode::And => "OpAnd",
            Opcode::Minus => "OpMinus",
            Opcode::Bang => "OpBang",
            Opcode::JumpNotTruthy => "OpJumpNotTruthy",
            Opcode::Jump => "OpJump",
            Opcode::Null => "OpNull",
            Opcode::SetGlobal => "OpSetGlobal",
            Opcode::GetGlobal => "OpGetGlobal",
            Opcode::SetLocal => "OpSetLocal",
            Opcode::GetLocal => "OpGetLocal",
            Opcode::Array => "OpArray",
            Opcode::HashMap => "OpHashMap",
            Opcode::Index => "OpIndex",
            Opcode::Call => "OpCall",
            Opcode::ReturnValue => "OpReturnValue",
            Opcode::Return => "OpReturn",
            Opcode::GetBuiltin => "OpBuiltIn",
            Opcode::Pop => "OpPop",
        };
        write!(f, "{op}")
    }
}

impl Opcode {
    pub fn lookup_widths(&self) -> Vec<u32> {
        match self {
            Opcode::Constant
            | Opcode::Jump
            | Opcode::JumpNotTruthy
            | Opcode::SetGlobal
            | Opcode::GetGlobal
            | Opcode::Array
            | Opcode::HashMap => vec![2],
            Opcode::Call | Opcode::SetLocal | Opcode::GetLocal | Opcode::GetBuiltin => vec![1],
            _ => vec![],
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn make(&self, operands: Vec<i32>) -> Instructions {
        let widths = self.lookup_widths();
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(*self as u8);

        for (operand, width) in operands.iter().zip(widths) {
            match width {
                2 => instructions
                    .write_u16::<BigEndian>(*operand as u16)
                    .unwrap(),
                1 => instructions.write_u8(*operand as u8).unwrap(),
                _ => panic!("Unrecognized operand width: {width}"),
            }
        }

        Instructions::new(instructions)
    }

    fn read_operands(widths: &Vec<u32>, ins: &[u8]) -> (Vec<i32>, i32) {
        let mut operands: Vec<i32> = Vec::new();
        let mut offset = 0;

        for width in widths {
            match width {
                2 => {
                    operands.push(i32::from(read_u16(&ins[offset..offset + 2])));
                    offset += 2;
                }
                1 => {
                    operands.push(i32::from(ins[offset]));
                    offset += 1;
                }
                _ => panic!("Unrecognized operand width: {width}"),
            }
        }

        (operands, offset as i32)
    }
}

/// This is a helper function to read a u16 from a byte slice, using
/// big endian encoding.
///
/// # Arguments
///
/// * `data` - A byte slice
///
/// # Returns
///
/// * `u16` - A u16
///
/// # Examples
///
/// ```
/// use compiler::code::read_u16;
/// let data = vec![0, 1];
/// let result = read_u16(&data);
/// assert_eq!(result, 1);
///
/// let data = vec![255, 255];
/// let result = read_u16(&data);
/// assert_eq!(result, 65535);
/// ```
pub fn read_u16(data: &[u8]) -> u16 {
    BigEndian::read_u16(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make() {
        let tests = vec![
            (
                Opcode::Constant,
                vec![65534],
                vec![Opcode::Constant as u8, 255, 254],
            ),
            (Opcode::Add, vec![], vec![Opcode::Add as u8]),
            (
                Opcode::GetLocal,
                vec![255],
                vec![Opcode::GetLocal as u8, 255],
            ),
        ];

        for (op, operands, expected) in tests {
            let instructions = op.make(operands);
            check_instruction(&expected, &instructions);
        }
    }

    fn check_instruction(expected: &Vec<u8>, actual: &Instructions) {
        let expected_len = expected.len();
        let actual_len = actual.data.len();

        assert_eq!(expected_len, actual_len);

        for (i, b) in expected.iter().enumerate() {
            assert_eq!(actual.data[i], *b);
        }
    }

    #[test]
    fn test_instructions_string() {
        let instructions = vec![
            Opcode::Add.make(vec![]),
            Opcode::GetLocal.make(vec![1]),
            Opcode::Constant.make(vec![2]),
            Opcode::Constant.make(vec![65535]),
        ];

        let mut test_instruction = Instructions::default();
        for instruction in instructions {
            test_instruction.append(instruction);
        }

        let expected = "0000 OpAdd\n0001 OpGetLocal 1\n0003 OpConstant 2\n0006 OpConstant 65535\n";

        assert_eq!(test_instruction.to_string(), expected);
    }

    #[test]
    fn test_read_operands() {
        let tests = vec![
            (Opcode::Constant, vec![65535], 2),
            (Opcode::GetLocal, vec![255], 1),
        ];

        for (op, operands, bytes_read) in tests {
            let instructions = op.make(operands.clone());
            let widths = op.lookup_widths();

            let (got_operands, offset) = Opcode::read_operands(&widths, &instructions.data[1..]);
            assert_eq!(offset, bytes_read, "offset wrong");
            assert!(got_operands.len() == operands.len(), "operands len wrong");
            assert_eq!(got_operands, operands, "operands wrong");
        }
    }
}
