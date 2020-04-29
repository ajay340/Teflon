pub mod vm;
pub mod instructions;
pub mod repl;
pub mod assembler;
pub mod lexer;
pub mod parser;

fn main(){
    let mut lex = lexer::Lexer::new();
    lex.lex_line("ADD #4 $3", 1);
    let ast = parser::parser::AbstractSyntaxTree::new(lex.tokens);
    &ast.parse();
    //print!("{:?}", ast);

    // let mut repl = repl::REPL::new();
    // repl.run();
}