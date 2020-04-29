use crate::lexer;
use std::fmt;

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    tokens: Vec<lexer::token::Token>,
    tree:   Vec<Binary>,
    index: usize,
}

#[derive(Debug)]
struct Binary {
    operator: lexer::token::TokenType,
    left: lexer::token::TokenType,
    right: lexer::token::TokenType,
}


impl Binary{
    fn new(op: lexer::token::TokenType, left: lexer::token::TokenType, right:lexer::token::TokenType) -> Binary{
        Binary{operator:op, left:left, right:right}
    }
}

impl AbstractSyntaxTree {
    pub fn new(lex_tokens: Vec<lexer::token::Token>) -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            tokens: lex_tokens,
            index: 0,
            tree: vec![],
        }
    }
    pub fn parse(mut self){
        for token in self.tokens{
            match &token.token{
                lexer::token::TokenType::EOF => println!("END OF FILE"),
                lexer::token::TokenType::IntOperand => println!("INTEGER INCOMING"),
                lexer::token::TokenType::NUMBER(NUMBER) => println!("NUMBER OP"),
                lexer::token::TokenType::OPCODE(ADD) => 
                    self.tree.push(Binary::new(token.token,lexer::token::TokenType::EOF, lexer::token::TokenType::EOF)),
                lexer::token::TokenType::REGISTER => println!("REGISTER INCOMING"),
                _ => println!("{:?}",token.token)
            }
        }
    }
}
// Instruction ::= <opcode> ‘$’ <register>  ‘$’<register>  ‘$’ <register>  | <opcode> ‘$’ <register> ‘#’ <int operand>  | <opcode> ‘#’ <int operand>
