use std::fmt::Display;

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

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
            let def = match Opcode::from_u8(self.data[i]) {
                Some(op) => op.lookup(),
                None => panic!("ERROR: Unknown opcode: {}", self.data[i]),
            };
            let (operands, read) = Opcode::read_operands(&def, &self.data[i + 1..]);
            writeln!(f, "{:04} {}", i, self.format_instruction(&def, &operands))?;
            i += 1 + read as usize;
        }
        Ok(())
    }
}

impl Instructions {
    pub fn new(data: Vec<u8>) -> Instructions {
        Instructions { data }
    }

    pub fn format_instruction(&self, def: &OperandDefinition, operands: &Vec<i32>) -> String {
        let operand_count = def.operand_widths.len();
        if operands.len() != operand_count {
            return format!(
                "ERROR: operand len {} does not match defined {}",
                operands.len(),
                operand_count
            );
        }

        match operand_count {
            1 => format!("{} {}", def.operand, operands[0]),
            0 => format!("{}", def.operand),
            _ => format!("Unhandeled operand_count for {}", def.operand),
        }
    }

    pub fn append(&mut self, mut new_instructions: Instructions) {
        self.data.append(&mut new_instructions.data);
    }
}

pub struct OperandDefinition {
    pub operand: Opcode,
    pub operand_widths: Vec<u32>,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Opcode {
    Constant,
    Add,
    Pop,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Opcode::Constant => "OpConstant",
            Opcode::Add => "OpAdd",
            Opcode::Pop => "OpPop",
        };
        write!(f, "{op}")
    }
}

impl Opcode {
    pub fn lookup(&self) -> OperandDefinition {
        match self {
            Opcode::Constant => OperandDefinition {
                operand: Opcode::Constant,
                operand_widths: vec![2],
            },
            Opcode::Add => OperandDefinition {
                operand: Opcode::Add,
                operand_widths: vec![],
            },
            Opcode::Pop => OperandDefinition {
                operand: Opcode::Pop,
                operand_widths: vec![],
            },
        }
    }

    pub fn make(&self, operands: Vec<i32>) -> Instructions {
        let def = self.lookup();
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(def.operand as u8);

        for (operand, width) in operands.iter().zip(def.operand_widths) {
            match width {
                2 => instructions
                    .write_u16::<BigEndian>(*operand as u16)
                    .unwrap(),
                _ => panic!("Unrecognized operand width: {width}"),
            }
        }

        Instructions::new(instructions)
    }

    fn read_operands(def: &OperandDefinition, ins: &[u8]) -> (Vec<i32>, i32) {
        let mut operands: Vec<i32> = Vec::new();
        let mut offset = 0;

        for width in def.operand_widths.iter() {
            match width {
                2 => {
                    operands.push(read_u16(&ins[offset..offset + 2]) as i32);
                    offset += 2;
                }
                _ => panic!("Unrecognized operand width: {width}"),
            }
        }

        (operands, offset as i32)
    }
}

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
            Opcode::Add.make(vec![1]),
            Opcode::Constant.make(vec![2]),
            Opcode::Constant.make(vec![65535]),
        ];

        let mut test_instruction = Instructions::default();
        for instruction in instructions {
            test_instruction.append(instruction);
        }

        let expected = "0000 OpAdd\n0001 OpConstant 2\n0004 OpConstant 65535\n";

        assert_eq!(test_instruction.to_string(), expected);
    }

    #[test]
    fn test_read_operands() {
        let tests = vec![(Opcode::Constant, vec![65535], 2)];

        for (op, operands, bytes_read) in tests {
            let instructions = op.make(operands.clone());
            let def = op.lookup();

            let (got_operands, offset) = Opcode::read_operands(&def, &instructions.data[1..]);
            assert_eq!(offset, bytes_read, "offset wrong");
            assert!(got_operands.len() == operands.len(), "operands len wrong");
            assert_eq!(got_operands, operands, "operands wrong");
        }
    }
}
