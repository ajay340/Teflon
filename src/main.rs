pub mod vm;
pub mod instructions;
pub mod repl;
pub mod assembler;
pub mod lexer;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}