use crate::lexer;

pub struct Parser {
    tokens: Vec<lexer::token::Token>,
    pub current: usize
}

impl Parser {
    pub fn new (tokens: Vec<lexer::token::Token>) -> Parser{
        return Parser {
            tokens: tokens, 
            current: 0
        }
    }
    fn isAtEnd(self) -> bool {      
        return match self.peek(){
            lexer::token::TokenType::EOF => true,
            _ => false,
        }
    }
    fn peek(&self) -> &lexer::token::TokenType{
        return &self.tokens.get(self.current).unwrap().token; 
    }
    fn previous(&self) -> &lexer::token::TokenType{
        return &self.tokens.get(self.current - 1).unwrap().token; 
    }
    fn next(&mut self){
       self.current += 1;
    }
}

// Instruction ::= <opcode> ‘$’ <register>  ‘$’<register>  ‘$’ <register>  | <opcode> ‘$’ <register> ‘#’ <int operand>  | <opcode> ‘#’ <int operand>
