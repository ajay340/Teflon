pub mod vm;
pub mod instructions;
pub mod repl;
pub mod assembler;
pub mod lexer;

fn main() {

    let mut lexer = lexer::Lexer::new();
    lexer.lex_file(String::from("src/instr.txt"));

    println!("{:?}", lexer.tokens);

    /*
    let mut repl = repl::REPL::new();
    repl.run();
    */
}