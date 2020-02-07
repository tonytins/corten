#![allow(dead_code)]
use crate::instruction::Opcode;

pub struct VM {
    /// Array that simulates the hardware register
    pub registers: [i32; 32],
    /// Program counter that tracks which byte is being executed
    pc: usize,
    /// The byte of the program being ran
    pub program: Vec<u8>,
    /// The remainer of the module used in the division opcode
    remainder: u32,
    /// The result of the last comparison operation
    equal_flag: bool,
}

impl VM {
    /// Creates and returns a new VM
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
        }
    }

    /// Attempts to decode the byte the VM's program is pointing at
    /// into an opcode
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    /// Attempts to decode the byte into an opcode
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    /// Grabs the next 16 bits (2 bytes)
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }

    /// Wraps the execuation in a loop so it will continue to run until done or
    /// there is an error executing the instructions.
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction()
        }
    }

    /// Executes one instruction. Meant to allow for more controlled of the VM.
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    /// Adds an byte to the VM's program
    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    /// Executes an instruction and returns a bool. Meant to be called by the
    /// various public functions.
    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let reg = self.next_8_bits() as usize;
                let num = self.next_16_bits() as u16;
                self.registers[reg] = num as i32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            }
            Opcode::IGL => {
                println!("Unrecognized opcode found! Terminating!");
                return false;
            }
            Opcode::ADD => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 + reg2;
            }
            Opcode::MUL => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 * reg2;
            }
            Opcode::DIV => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 / reg2;
                self.remainder = (reg1 % reg2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::SUB => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 - reg2;
            }
            Opcode::JMPF => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc += target as usize;
            }
            Opcode::JMPB => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc -= target as usize;
            }
            Opcode::EQ => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];

                if reg1 == reg2 {
                    self.equal_flag = true
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            }
            Opcode::NEQ => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];

                if reg1 != reg2 {
                    self.equal_flag = true
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            }
            Opcode::GT => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];

                if reg1 > reg2 {
                    self.equal_flag = true
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            }
            Opcode::LT => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];

                if reg1 < reg2 {
                    self.equal_flag = true
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            }
            Opcode::GTE => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];

                if reg1 >= reg2 {
                    self.equal_flag = true
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            }
            Opcode::LTE => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];

                if reg1 <= reg2 {
                    self.equal_flag = true
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            }
            Opcode::JMPE => {
                let reg = self.next_8_bits() as usize;
                let target = self.registers[reg];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::NOP => {
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
            }
        }
        true
    }
}

#[cfg(test)]
mod vm_tests {
    use crate::vm::*;

    fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;

        test_vm
    }

    #[test]
    fn test_crate_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_hlt_opcode() {
        let mut vm = get_test_vm();
        let test_bytes = vec![5, 0, 0, 0];
        vm.program = test_bytes;
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_add_opcode() {
        let mut vm = get_test_vm();
        vm.program = vec![1, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[2], 15);
    }

    #[test]
    fn test_sub_opcode() {
        let mut vm = get_test_vm();
        vm.program = vec![2, 1, 0, 2];
        vm.run();
        assert_eq!(vm.registers[2], 5);
    }

    #[test]
    fn test_div_opcode() {
        let mut vm = get_test_vm();
        vm.program = vec![4, 1, 0, 2];
        vm.run();
        assert_eq!(vm.registers[2], 2);
    }

    #[test]
    fn test_eq_opcode() {
        let mut vm = get_test_vm();
        vm.registers[0] = 10;
        vm.registers[1] = 10;
        vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        vm.registers[1] = 20;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut vm = get_test_vm();
        vm.registers[0] = 10;
        vm.registers[1] = 20;
        vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
    }

    #[test]
    fn test_gte_opcode() {
        let mut vm = get_test_vm();
        vm.registers[0] = 20;
        vm.registers[1] = 10;
        vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0, 11, 0, 1, 0];
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        vm.registers[0] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        vm.registers[0] = 5;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
    }

    #[test]
    fn test_gt_opcode() {
        let mut vm = get_test_vm();
        vm.registers[0] = 20;
        vm.registers[1] = 10;
        vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 0, 1, 0];
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        vm.registers[0] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        vm.registers[0] = 5;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
    }

    #[test]
    fn test_lte_opcode() {
        let mut vm = get_test_vm();
        vm.registers[0] = 20;
        vm.registers[1] = 10;
        vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0];
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        vm.registers[0] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        vm.registers[0] = 5;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
    }

    #[test]
    fn test_lt_opcode() {
        let mut vm = get_test_vm();
        vm.registers[0] = 20;
        vm.registers[1] = 10;
        vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        vm.registers[0] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        vm.registers[0] = 5;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
    }

    #[test]
    fn test_igl_opcode() {
        let mut vm = get_test_vm();
        let test_bytes = vec![254, 0, 0, 0];
        vm.program = test_bytes;
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut vm = get_test_vm();
        vm.registers[0] = 1;
        vm.program = vec![6, 0, 0, 0];
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut vm = get_test_vm();
        vm.registers[0] = 2;
        vm.program = vec![7, 0, 0, 0, 5, 0, 0, 0];
        vm.run_once();
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut vm = get_test_vm();
        vm.registers[1] = 6;
        vm.program = vec![0, 0, 0, 10, 8, 1, 0, 0];
        vm.run_once();
        vm.run_once();
        assert_eq!(vm.pc, 0);
    }

    #[test]
    fn test_jmpe_opcode() {
        let mut vm = get_test_vm();
        vm.registers[0] = 7;
        vm.equal_flag = true;
        vm.program = vec![15, 0, 0, 0, 15, 0, 0, 0, 15, 0, 0, 0];
        vm.run_once();
        assert_eq!(vm.pc, 7);
    }

    #[test]
    fn test_load_opcode() {
        let mut vm = get_test_vm();
        // 500 is represented this way in little endian format
        vm.program = vec![0, 0, 1, 244];
        vm.run();
        assert_eq!(vm.registers[0], 500);
    }
}
