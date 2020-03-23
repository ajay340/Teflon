pub mod vm;
pub mod instructions;
pub mod repl;
pub mod assembler;
pub mod lexer;

fn main() {
    //let mut repl = repl::REPL::new();
    //repl.run();


    let mut l = lexer::Lexer::new("LOAD $10 #786");
    let v = l.lex();

    println!("{:?}", v);
}
