#![allow(unused_imports)]
use nom::*;
use nom::{digit, types::CompleteStr};

use crate::assembler::Token;

named!(pub integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                Token::Number{value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);

#[cfg(test)]
mod reg_parser_test {
    use super::*;

    #[test]
    fn test_opcode() {
        let result = integer_operand(CompleteStr("#10"));
        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::Number { value: 10 });
    }
}
