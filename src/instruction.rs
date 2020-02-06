#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    IGL
}


impl From<u8> for Opcode {
    fn from(vm: u8) -> Self {
        match vm {
            0 => Opcode::HLT,
            _ => Opcode::IGL
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction {
            opcode
        }
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