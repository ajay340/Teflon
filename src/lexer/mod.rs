use std::fs;
use std::iter::Peekable;

pub struct Lexer {
    line: String,
    tokens: Vec<Token>,
}

pub enum Token {
    Operator(String),
    Register(u8),
    NumberToken(u8),
}


// LOAD $1 #10
// ADD $1 $2 $3
impl Lexer {
    pub fn new(line: &str) -> Lexer {
        Lexer {
            line: String::from(line),
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Vec<String> {
        
        match Self::find_lexemes(&self.line) {
            Ok(res) => res,
            Err(_e) =>  {
                Vec::new()
            },
        }
    }

    fn lex_line(&self) -> Vec<String> {
        match Self::find_lexemes(&self.line) {
            Ok(res) => res,
            Err(_e) =>  {
                Vec::new()
            },
        }
    }

    fn find_lexemes(line: &String) -> Result<Vec<String>, String> {
        let mut lexemes: Vec<String> = Vec::new();
        let mut it = line.chars().peekable();

        while let Some(&c) = it.peek() {
            match c {
                'a'..='z' |  'A'..='Z' => {
                    it.next();
                    let op = Self::get_operator(c, &mut it);
                    lexemes.push(op)
                },
                '$' => {
                    it.next();
                    lexemes.push(c.to_string());
                },
                '#' => {
                    it.next();
                    lexemes.push(c.to_string());
                },
                '0'..='9' => {
                    it.next();
                    lexemes.push(Self::get_number(c, &mut it));
                },
                ' ' => {
                    it.next();
                },
                _ => return Err(format!("Unexpected character {}", c)),
            }
        }
        Ok(lexemes)
    }

    fn get_number<T: Iterator<Item = char>>(num: char, it: &mut Peekable<T>) -> String {
        let mut full_number = String::from(num.to_string());
        while let Some(&x) = it.peek() {
            it.next(); // increment the iterator
            match x {
                '0'..='9' =>  {
                    full_number.push(x);
                }
                _ => break
            }
        }
        full_number
    }

    fn get_operator<T: Iterator<Item = char>>(c: char, it: &mut Peekable<T>) -> String {
        let mut operator = String::from(c.to_string());
        while let Some(&c) = it.peek() {
            match c {
                'a'..='z' |  'A'..='Z' => {
                    it.next();
                    operator.push(c)
                },
                _ => break,
            }
        }
        operator
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_lexemes_for_load_instruction() {
        let test_lexer = Lexer::new("LOAD $1 #1000");
        let lexemes = test_lexer.lex_line();
        let proper_lexemes = vec!["LOAD", "$", "1", "#", "1000"];
        assert_eq!(lexemes, proper_lexemes);
    }

    fn test_get_lexemes_for_add_instruction() {
        let test_lexer = Lexer::new("ADD $1 $2 $3");
        let lexemes = test_lexer.lex_line();
        let proper_lexemes = vec!["ADD", "$", "1", "$", "2", "$", "3"];
        assert_eq!(lexemes, proper_lexemes);
    }

}