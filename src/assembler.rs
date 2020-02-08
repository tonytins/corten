#![allow(dead_code)]
pub mod instruction_parser;
pub mod opcode_parser;
pub mod operand_parser;
pub mod program_parser;
pub mod register_parser;

use crate::instruction::Opcode;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Opcode { code: Opcode },
    Register { reg_num: u8 },
    Number { value: i32 },
}
