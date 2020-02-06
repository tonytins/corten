use crate::instruction::Opcode::LOAD;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    IGL,
    LOAD,
    ADD,
    MUL,
    DIV,
    JMP,
}

impl From<u8> for Opcode {
    fn from(vm: u8) -> Self {
        match vm {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::JMP,
            3 => Opcode::ADD,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
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
