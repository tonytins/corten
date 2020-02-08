use std;
use std::io;
use std::io::Write;

use crate::vm::VM;
use crate::assembler::program_parser::program;
use metacrate::crate_version;
use rbtag::{BuildDateTime, BuildInfo};
use nom::types::CompleteStr;

#[derive(BuildDateTime, BuildInfo)]
struct BuildTag;

/// Remove "-clean" from the commit id
fn normalize_commit_id(id: &str) -> String {
    let clean_stat = "-clean";

    match id.contains(clean_stat) {
        true => id.replace(clean_stat, ""),
        false => id.to_string(),
    }
}

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
        let ver_id = format!(
            "{}+{}",
            crate_version!(),
            normalize_commit_id(BuildTag {}.get_build_commit())
        );
        println!("Corten {}", ver_id);
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
                }
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
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                }
                _ => {
                    let prog = match program(buffer.into()) {
                        Ok((_, prog)) => prog,
                        Err(_) => {
                            println!("Unable to parse input.");
                            continue;
                        }
                    };

                    self.vm.program.append(&mut prog.to_bytes());
                }
            }
        }
    }
}
