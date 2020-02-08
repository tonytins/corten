use nom::*;
use crate::assembler::Token;
use crate::assembler::opcode_parser::opcode_load;
use crate::assembler::operand_parser::integer_operand;
use crate::assembler::register_parser::register;

use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    op1: Option<Token>,
    op2: Option<Token>,
    op3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];

        match self.opcode.to_owned() {
            Token::Opcode { code } => match code {
                _ => {
                    results.push(code.into());
                }
            },
            _ => {
                println!("Incorrect opcode!");
                std::process::exit(1);
            }
        };


        for operand in vec![&self.op1, &self.op2, &self.op3] {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut results),
                None => {}
            }
        }

        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            },
            Token::Number { value } => {
                let conv = *value as u16;
                let byte1 = conv;
                let byte2 = conv >> 8;

                results.push(byte2 as u8);
                results.push(byte1 as u8);
            },
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }

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
    use crate::assembler::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
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
