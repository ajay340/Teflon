mod token;
use token::{ Token, TokenType, Error, LexerError };
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Lexer {
    state: State,
    val: String,
    pub tokens: Vec<Token>,
    errors: Vec<LexerError>
}    
    
#[derive(Debug, PartialEq)]  
pub enum State {
    S,      // S => No pattern has been detected
    D,      // D => part of a number has been detected
    O,      // O => part of a opcode has been detected
    C,      // C => part of a comment has been detected
}


impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            state: State::S,
            val: String::from(""),
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn lex_file(&mut self, path: String) {
        let f = File::open(path).expect("File does not exist");
        let buf_reader = BufReader::new(f);        
        let mut line_number = 1;

        for line in buf_reader.lines() {
            match line {
                Ok(line) => self.lex_line(&line, line_number),
                Err(e) => panic!("Error reading line: {}", e),
            }
            line_number += 1;
        }
        self.tokens.push(Token::new(TokenType::EOF, line_number));
    }

    pub fn lex_line(&mut self, line: &str, line_number: usize) {
        let mut it = line.chars().peekable();
        while let Some(val) = it.next() {
            if it.peek() == None {
                self.final_iteration(val, line_number);
            } else {
                self.next_state(val, line_number);
            }   
        }
    }


    pub fn lex_debug(&self) {
        for token in &self.tokens {
            println!("{:?}", token);
        }
    }

    #[allow(dead_code)]
    // Lex's a single line. This function is just used for testing
    fn lex_single_line(&mut self, line: &str) {
        self.lex_line(line, 1);
        self.tokens.push(Token::new(TokenType::EOF, 1));
    }

    fn next_state(&mut self, c: char, line_number: usize) {
        match self.state {
            State::S => self.s_state_transition(c, line_number),
            State::D => self.d_state_transition(c, line_number),
            State::O => self.o_state_transition(c, line_number),
            State::C => self.c_state_transition(c),
        }
    }

    fn s_state_transition(&mut self, c: char, line: usize) {
        match c {
            'a'..='z' | 'A'..='Z' => {
                self.val.push(c);
                self.state = State::O;
            },
            '1'..='9' => {
                self.val.push(c);
                self.state = State::D;
            },
            '$' => self.add_token(TokenType::REGISTER, line),
            '#' => self.add_token(TokenType::IntOperand, line),
            '<' => self.state = State::C,
            '\n' | '\r' | ' ' => (),
            '>' => self.errors.push(LexerError::new(Error::CommentError(line))),
            _ => self.errors.push(LexerError::new(Error::TokenError(line, c))),
        }
    }

    // A opcode has been detected
    fn o_state_transition(&mut self, c: char, line: usize) {
        match c {
            'a'..='z' | 'A'..='Z' => self.val.push(c),
            _ => self.reset_and_add_token(TokenType::OPCODE(self.val.clone()), line, c),
        }
    }

    // An integer has been detected
    fn d_state_transition(&mut self, c: char, line: usize) {
        match c {
            '0'..='9' => self.val.push(c),
            _ => self.reset_and_add_token(TokenType::NUMBER(self.val.clone()), line, c),
        }
    }

    fn c_state_transition(&mut self, c: char) {
        match c {
            '>' => self.state = State::S,
            _ => (),
        }
    }

    fn final_iteration(&mut self, c: char, line: usize) {
        self.next_state(c, line);
        match self.state {
            State::S => (),
            State::D => self.add_token(TokenType::NUMBER(self.val.clone()), line),
            State::O => self.add_token(TokenType::OPCODE(self.val.clone()), line),
            State::C => {
                if c != '>' {
                    self.errors.push(LexerError::new(Error::CommentError(line)))
                }
                ()
            },
        }
    }

    fn add_token(&mut self, token_type: TokenType, line: usize) {
        self.tokens.push(Token::new(token_type, line));
    }

    fn reset_and_add_token(&mut self, token_type: TokenType, line: usize, c: char) {
        self.tokens.push(Token::new(token_type, line));
        self.reset_values();
        self.next_state(c, line);
    }

    fn reset_values(&mut self) {
        self.val= String::from("");
        self.state = State::S;
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[macro_use]
    macro_rules! to_String {
        ($e:expr) => {
            String::from($e)
        };
    }


    #[test]
    fn test_get_lexemes_for_load_instruction() {
        let mut test_lexer = Lexer::new();
        test_lexer.lex_single_line("LOAD $1 #1000");
        let tokens = vec![
            Token::new(TokenType::OPCODE(to_String!("LOAD")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("1")), 1),
            Token::new(TokenType::IntOperand, 1),
            Token::new(TokenType::NUMBER(to_String!("1000")), 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(test_lexer.tokens, tokens);
    }

    #[test]
    fn test_get_lexemes_for_add_instruction() {
        let mut test_lexer = Lexer::new();
        test_lexer.lex_single_line("ADD $11 $2 $3");
        let tokens = vec![
            Token::new(TokenType::OPCODE(to_String!("ADD")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("11")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("2")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("3")), 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(test_lexer.tokens, tokens);
    }

    #[test]
    fn test_comment() {
        let mut test_lexer = Lexer::new();
        test_lexer.lex_single_line("< Hello for a comment >");
        let tokens = vec![
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(test_lexer.tokens, tokens);
    }

    #[test]
    fn test_comment_with_code() {
        let mut test_lexer = Lexer::new();
        test_lexer.lex_single_line("ADD $11 $2 $3 <this code should work>");
        let tokens = vec![
            Token::new(TokenType::OPCODE(to_String!("ADD")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("11")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("2")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("3")), 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(test_lexer.tokens, tokens);
    }

    #[test]
    fn test_invalid_symbol_percent() {
        let mut test_lexer = Lexer::new();
        test_lexer.lex_single_line("ADD $11 $2% $3 <this code should work>");
        let tokens = vec![
            Token::new(TokenType::OPCODE(to_String!("ADD")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("11")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("2")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("3")), 1),
            Token::new(TokenType::EOF, 1),
        ];
        let errors = vec![LexerError::new(Error::TokenError(1, '%'))];
        assert_eq!(test_lexer.tokens, tokens);
        assert_eq!(test_lexer.errors, errors);
    }

    #[test]
    fn test_invalid_comment_error() {
        let mut test_lexer = Lexer::new();
        test_lexer.lex_single_line("ADD $11 $2 $3 <this code should work");
        let tokens = vec![
            Token::new(TokenType::OPCODE(to_String!("ADD")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("11")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("2")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("3")), 1),
            Token::new(TokenType::EOF, 1),
        ];
        let errors = vec![LexerError::new(Error::CommentError(1))];

        assert_eq!(test_lexer.tokens, tokens);
        assert_eq!(test_lexer.errors, errors);
    }
}