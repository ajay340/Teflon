//use std::fs;
use std::iter::Peekable;


pub struct Lexer {
    state: State,
    val: String,
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Opcode(String),
    Register(String),
    IntOperand(String),
}


    
    
    
   
pub enum State {
    S,      /// S => No pattern has been detected
    H,      /// H => Part of a Integer operand has been detected
    D,      /// D => Part of a Register has been detected
    O,      /// O => Part of a Opcode has been detected 
    X,      /// An Opcode was detected
    Y,      /// A Register was detected
    Z,      // A Integer Operand was detected
}


impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            state: State::S,
            val: String::from(""),
            tokens: Vec::new(),
        }
    }

    pub fn lex_line(&mut self, line: &str) {
        let mut it = line.chars().peekable();

        while let Some(&val) = it.peek() {
            Self::next_state(self, val, &mut it);
        }
    }


    fn next_state<I: Iterator<Item= char>>(&mut self, c: char, it: &mut Peekable<I>) {
        match self.state {
            State::S => Self::s_state_transition(self, c, it),
            State::H => Self::h_state_transition(self, it),
            State::D => Self::d_state_transition(self, it),
            State::O => Self::o_state_transition(self, it),
            State::X => Self::reset_and_add_token(self, Token::Opcode(self.val.clone())),
            State::Y => Self::reset_and_add_token(self, Token::Register(self.val.clone())),
            State::Z => Self::reset_and_add_token(self, Token::IntOperand(self.val.clone())),
        }
    }

    fn s_state_transition<I: Iterator<Item=char>>(&mut self, c: char, it: &mut Peekable<I>) {
        match c {
            'a'..='z' | 'A'..='Z' => self.state = State::O,
            '$' => {
                it.next();
                self.state = State::D;
            },
            '#' => {
                it.next();
                self.state = State::H;
            },
            _ => panic!("Invalid symbol {}, for state transition", c),
        }
    }
    
    fn h_state_transition<I: Iterator<Item= char>>(&mut self, it: &mut Peekable<I>) {
        if let Some(c) = it.next() {
            match c {
                '0'..='9' => self.val.push(c),
                '\n' => Self::reset_and_add_token(self, Token::IntOperand(self.val.clone())),
                _ => panic!("Invalid symbol {}, at current position for integer operand", c),
            }
        } else {
            panic!("Iterator does not have a next");
        }
    }

    fn o_state_transition<I: Iterator<Item= char>>(&mut self, it: &mut Peekable<I>) {
        if let Some(c) = it.next() {
            match c {
                'a'..='z' | 'A'..='Z' => self.val.push(c),
                ' ' => self.state = State::X,
                _ => panic!("Invalid symbol {}, at current position for opcode", c),
            }
        } else  {
            panic!("Iterator does not have a next");
        }
    }


    fn d_state_transition<I: Iterator<Item= char>>(&mut self, it: &mut Peekable<I>) {
        if let Some(c) = it.next() {
            match c {
                '0'..='9' => self.val.push(c),
                ' ' => self.state = State::Y,
                '\n' => Self::reset_and_add_token(self, Token::Register(self.val.clone())),
                _ => panic!("Invalid symbol {}, at current position for register", c),
            }
        } else {
            panic!("Iterator does not have a next");
        }
    }

    fn reset_and_add_token(&mut self, token: Token) {
        self.tokens.push(token);
        self.val = String::from("");
        self.state = State::S
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
        test_lexer.lex_line("LOAD $1 #1000\n");
        let tokens = vec![
            Token::Opcode(to_String!("LOAD")),
            Token::Register(to_String!("1")),
            Token::IntOperand(to_String!("1000"))
        ];
        assert_eq!(test_lexer.tokens, tokens);
    }

    #[test]
    fn test_get_lexemes_for_add_instruction() {
        let mut test_lexer = Lexer::new();
        test_lexer.lex_line("ADD $11 $2 $3\n");
        let tokens = vec![
            Token::Opcode(to_String!("ADD")),
            Token::Register(to_String!("11")),
            Token::Register(to_String!("2")),
            Token::Register(to_String!("3"))
        ];
        assert_eq!(test_lexer.tokens, tokens);
    }
}