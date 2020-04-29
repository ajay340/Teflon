use std::fmt::{Display, Formatter, Debug, Result};
use std::error;

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
    pub token: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(token: TokenType, line: usize) -> Token {
        Token {
            token,
            line,
        }
    }
}

#[derive(PartialEq)]
pub enum Error {
    TokenError(usize, char),
    CommentError(usize),
}

#[derive(PartialEq)]
pub struct LexerError {
    err: Error
}

impl LexerError {
    pub fn new(err: Error) -> LexerError {
        LexerError {
            err
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.err {
            Error::TokenError(line, c) => write!(f, "An error occurred lexing token {} on line {}", line, c),
            Error::CommentError(line) => write!(f, "Invalid comment block on line {}", line),
        }
    }
}

impl Debug for LexerError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}