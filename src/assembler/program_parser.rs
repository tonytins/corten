use nom::types::CompleteStr;
use nom::*;

use crate::assembler::instruction_parser::{instruction_one, AssemblerInstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut prog = vec![];
        for instruction in &self.instructions {
            prog.append(&mut instruction.to_bytes());
        }
        prog
    }
}

named!(pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(instruction_one) >>
        (
            Program {
                instructions
            }
        )
    )
);

#[cfg(test)]
mod instruction_parser_test {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let result = program(CompleteStr("load $0 #100\n"));
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, CompleteStr(""));
        assert_eq!(1, p.instructions.len());
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program(CompleteStr("load $0 #100\n"));
        let (_, prog) = result.unwrap();
        let bytecode = prog.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}
