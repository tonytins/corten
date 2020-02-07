use std;
use std::io;
use std::num::ParseIntError;
use std::io::Write;

use crate::vm::VM;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    /// Runs a similar VM execution loop but the instructions are taken from
    /// the user directly at the terminal and not from pre-compiled byte code
    pub fn run(&mut self) {
        println!("Welcome to the Corten REPL");
        loop {
            // Allocates a new string in which to store the user types each iteration
            let mut buffer = String::new();

            // Blocking call until the user types in a command
            let stdin = io::stdin();
            print!(">>> ");

            // Look at the string the user gave us
            io::stdout().flush().expect("Unable to flush stdout.");
            stdin.read_line(&mut buffer).expect("Unable to read line.");

            let buffer = buffer.trim();

            match buffer {
                ".exit" => {
                    std::process::exit(0);
                },
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".program" => {
                    println!("Listing instructions currently in VM's program:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                },
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                },
                _ => {
                    let results = self.parse_hex(buffer);
                    match results {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte);
                            }
                        }
                        Err(_er) => {
                            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.");
                        }
                    }
                    self.vm.run_once();
                }
            }
        }
    }

    /// Accepts the hexadecimal  without a leading '0x' and returns a Vec of a
    /// u8. Example: 00 01 03 E8
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                },
                Err(err) => {
                    return Err(err)
                }
            }
        }
        Ok(results)
    }
}