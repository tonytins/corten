#![allow(unused_imports)]
use nom::*;
use nom::{digit, types::CompleteStr};

use crate::assembler::Token;
use crate::instruction::Opcode;

named!(pub opcode_load<CompleteStr, Token>,
    do_parse!(tag!("load") >> (Token::Opcode{code: Opcode::LOAD}))
);

#[cfg(test)]
mod opcode_parser_test {
    use super::*;

    #[test]
    fn test_parser_op_load() {
        // Test that opcode is dected and parsed correctly
        let result = opcode_load(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Opcode { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));
    }
}
