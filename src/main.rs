pub mod vm;
pub mod instructions;
pub mod repl;
pub mod assembler;
pub mod lexer;
pub mod parser;

fn main(){
    let test = "add $1 2";
    println!("{:?}", parser::parser::tokenize(test));

    // let mut repl = repl::REPL::new();
    // repl.run();
}