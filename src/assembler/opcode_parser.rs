#![allow(unused_imports)]
use nom::*;
use nom::{alpha1, digit, types::CompleteStr};

use crate::assembler::{Opcode, Token};

named!(pub opcode<CompleteStr, Token>,
  do_parse!(
      opcode: alpha1 >>
      (
        {
            Token::Opcode{code: Opcode::from(opcode)}
        }
      )
  )
);

#[cfg(test)]
mod opcode_parser_test {
    use super::*;

    #[test]
    fn test_parser_op_load() {
        // Test that opcode is dected and parsed correctly
        let result = opcode(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Opcode { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));
        let result = opcode(CompleteStr("alod"));
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Opcode { code: Opcode::IGL });
    }
}
