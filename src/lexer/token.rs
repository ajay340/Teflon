
#[derive(Debug, PartialEq)]
pub enum TokenType {    // EX:
    OPCODE(String),     // Load
    NUMBER(String),     // 23
    REGISTER,           // $
    IntOperand,         // #
    EOF,                // End of file
}


#[derive(Debug, PartialEq)]
pub struct Token {
    token: TokenType,
    line: usize,
}

impl Token {
    pub fn new(token: TokenType, line: usize) -> Token {
        Token {
            token,
            line,
        }
    }
}