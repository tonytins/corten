mod instruction;
mod vm;
mod repl;

use repl::REPL;

fn main() {
    let mut repl = REPL::new();
    repl.run();
}
