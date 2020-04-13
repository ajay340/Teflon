mod token;
use token::{ Token, TokenType };

#[derive(Debug, PartialEq)]
pub struct Lexer {
    state: State,
    val: String,
    tokens: Vec<Token>,
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
        }
    }

    pub fn lex_line(&mut self, line: &str) {
        let mut it = line.chars().peekable();
        let mut line_number = 1;
        while let Some(val) = it.next() {
            if val == '\n' {
                line_number+=1;
            } else if it.peek() == None && val != ';' {
                self.final_iteration(val, line_number);
            } else {
                self.next_state(val, line_number);
            }   
        }
        println!("{:?}", self);
        self.tokens.push(Token::new(TokenType::EOF, line_number));
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
            ';' => self.add_token(TokenType::SEMICOLON, line),
            '#' => self.add_token(TokenType::IntOperand, line),
            '<' => self.state = State::C,
            '>' => (), //TODO: Better implementation
            ' ' => (),
            _ => panic!("Invalid symbol {} for state transition", c),
        }
    }

    // A opcode has been detected
    fn o_state_transition(&mut self, c: char, line: usize) {
        match c {
            'a'..='z' | 'A'..='Z' => self.val.push(c),
            ';' => self.reset_and_add_token_with_semicolon(TokenType::OPCODE(self.val.clone()), line),
            _ => self.reset_and_add_token(TokenType::OPCODE(self.val.clone()), line),
        }
    }

    // An integer has been detected
    fn d_state_transition(&mut self, c: char, line: usize) {
        match c {
            '0'..='9' => self.val.push(c),
            ';' => self.reset_and_add_token_with_semicolon(TokenType::NUMBER(self.val.clone()), line),
            _ => self.reset_and_add_token(TokenType::NUMBER(self.val.clone()), line),
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
                    panic!("Unterminated Comment block")
                }
                ()
            },
        }
    }

    fn add_token(&mut self, token_type: TokenType, line: usize) {
        self.tokens.push(Token::new(token_type, line));
    }

    fn reset_and_add_token(&mut self, token_type: TokenType, line: usize) {
        self.tokens.push(Token::new(token_type, line));
        self.reset_values();
    }

    fn reset_and_add_token_with_semicolon(&mut self, token_type: TokenType, line: usize) {
        self.reset_and_add_token(token_type, line);
        self.tokens.push(Token::new(TokenType::SEMICOLON, line));
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
        test_lexer.lex_line("LOAD $1 #1000;");
        let tokens = vec![
            Token::new(TokenType::OPCODE(to_String!("LOAD")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("1")), 1),
            Token::new(TokenType::IntOperand, 1),
            Token::new(TokenType::NUMBER(to_String!("1000")), 1),
            Token::new(TokenType::SEMICOLON, 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(test_lexer.tokens, tokens);
    }

    #[test]
    fn test_get_lexemes_for_load_instruction_without_semicolon() {
        let mut test_lexer = Lexer::new();
        test_lexer.lex_line("LOAD $1 #1000");
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
        test_lexer.lex_line("ADD $11 $2 $3;");
        let tokens = vec![
            Token::new(TokenType::OPCODE(to_String!("ADD")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("11")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("2")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("3")), 1),
            Token::new(TokenType::SEMICOLON, 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(test_lexer.tokens, tokens);
    }

    #[test]
    fn test_comment() {
        let mut test_lexer = Lexer::new();
        test_lexer.lex_line("< Hello for a comment >");
        let tokens = vec![
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(test_lexer.tokens, tokens);
    }

    #[test]
    fn test_comment_with_code() {
        let mut test_lexer = Lexer::new();
        test_lexer.lex_line("ADD $11 $2 $3; <this code should work>");
        let tokens = vec![
            Token::new(TokenType::OPCODE(to_String!("ADD")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("11")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("2")), 1),
            Token::new(TokenType::REGISTER, 1),
            Token::new(TokenType::NUMBER(to_String!("3")), 1),
            Token::new(TokenType::SEMICOLON, 1),
            Token::new(TokenType::EOF, 1),
        ];
        assert_eq!(test_lexer.tokens, tokens);
    }

}