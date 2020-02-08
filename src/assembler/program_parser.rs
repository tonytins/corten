use nom::types::CompleteStr;
use nom::*;

use crate::assembler::instruction_parser::{instruction_one, AssemblerInstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
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
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction() {
        let result = program(CompleteStr("load $0 #100\n"));
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, CompleteStr(""));
        assert_eq!(1, p.instructions.len());
    }
}
