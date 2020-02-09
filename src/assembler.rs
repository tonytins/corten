#![allow(dead_code)]

use nom::types::CompleteStr;

pub mod instruction_parser;
pub mod opcode_parser;
pub mod operand_parser;
pub mod program_parser;
pub mod register_parser;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Opcode { code: Opcode },
    Register { reg_num: u8 },
    Number { value: i32 },
}

#[derive(Debug, PartialEq, Clone)]
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
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GTE,
            12 => Opcode::GT,
            13 => Opcode::LTE,
            14 => Opcode::LT,
            15 => Opcode::JMPE,
            16 => Opcode::NOP,
            _ => Opcode::IGL,
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::LOAD => 0,
            Opcode::ADD => 1,
            Opcode::SUB => 2,
            Opcode::MUL => 3,
            Opcode::DIV => 4,
            Opcode::HLT => 5,
            Opcode::JMP => 6,
            Opcode::JMPF => 7,
            Opcode::JMPB => 8,
            Opcode::EQ => 9,
            Opcode::NEQ => 10,
            Opcode::GTE => 11,
            Opcode::LTE => 12,
            Opcode::LT => 13,
            Opcode::GT => 14,
            Opcode::JMPE => 15,
            Opcode::NOP => 16,
            Opcode::IGL => 100,
        }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        match v {
            CompleteStr("load") => Opcode::LOAD,
            CompleteStr("add") => Opcode::ADD,
            CompleteStr("sub") => Opcode::SUB,
            CompleteStr("mul") => Opcode::MUL,
            CompleteStr("div") => Opcode::DIV,
            CompleteStr("hlt") => Opcode::HLT,
            CompleteStr("jmp") => Opcode::JMP,
            CompleteStr("jmpf") => Opcode::JMPF,
            CompleteStr("jmpb") => Opcode::JMPB,
            CompleteStr("eq") => Opcode::EQ,
            CompleteStr("neq") => Opcode::NEQ,
            CompleteStr("gte") => Opcode::GTE,
            CompleteStr("gt") => Opcode::GT,
            CompleteStr("lte") => Opcode::LTE,
            CompleteStr("lt") => Opcode::LT,
            CompleteStr("jmpe") => Opcode::JMPE,
            CompleteStr("nop") => Opcode::NOP,
            _ => Opcode::IGL,
        }
    }
}
