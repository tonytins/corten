mod assembler;
mod instruction;
mod repl;
mod vm;

use repl::REPL;

fn main() {
    let mut repl = REPL::new();
    repl.run();
}
