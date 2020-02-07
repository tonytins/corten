#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    IGL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    /// Equal
    EQ,
    /// Not equal
    NEQ,
    /// Greater then
    GT,
    /// Less then
    LT,
    /// Greater then or equal to
    GTE,
    /// less then or equal
    LTE,
    /// jump if equal
    JMPE,
    /// Jump
    JMP,
    /// Jump forward
    JMPF,
    /// Jump backward
    JMPB,
    NOP,
}

impl From<u8> for Opcode {
    fn from(vm: u8) -> Self {
        match vm {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            6 => Opcode::HLT,
            7 => Opcode::JMP,
            8 => Opcode::JMPF,
            9 => Opcode::JMPB,
            10 => Opcode::EQ,
            11 => Opcode::NEQ,
            12 => Opcode::GTE,
            13 => Opcode::GT,
            14 => Opcode::LTE,
            15 => Opcode::LT,
            16 => Opcode::JMPE,
            17 => Opcode::NOP,
            _ => Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod instruction_tests {
    use crate::instruction::*;

    #[test]
    fn test_crate_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let inst = Instruction::new(Opcode::HLT);
        assert_eq!(inst.opcode, Opcode::HLT);
    }
}
