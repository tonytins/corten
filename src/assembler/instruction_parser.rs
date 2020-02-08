use nom::*;
use crate::assembler::Token;
use crate::assembler::opcode_parser::opcode_load;
use crate::assembler::operand_parser::integer_operand;
use crate::assembler::register_parser::register;

use nom::types::CompleteStr;
use nom::*;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    op1: Option<Token>,
    op2: Option<Token>,
    op3: Option<Token>,
}

named!(pub instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode_load >>
        r: register >>
        i: integer_operand >>
        (
            AssemblerInstruction{
                opcode: o,
                op1: Some(r),
                op2: Some(i),
                op3: None
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
        let result = instruction_one(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Opcode { code: Opcode::LOAD },
                    op1: Some(Token::Register { reg_num: 0 }),
                    op2: Some(Token::Number { value: 100 }),
                    op3: None
                }
            ))
        );
    }
}
