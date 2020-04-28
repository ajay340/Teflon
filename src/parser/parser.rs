use crate::lexer;

fn grammar(expr:&str, line:usize) ->lexer::token::Token{
    match expr {
        "add" => lexer::token::Token::new(lexer::token::TokenType::OPCODE(lexer::makeString("ADD")), line),
        expr if expr.chars().next().unwrap() == '$' => lexer::token::Token::new(lexer::token::TokenType::REGISTER, line),
        expr if expr.chars().next().unwrap() == '#' => lexer::token::Token::new(lexer::token::TokenType::IntOperand, line),
        expr if String::from(expr).parse::<f64>().is_ok() => lexer::token::Token::new(lexer::token::TokenType::NUMBER(lexer::makeString(expr)), line),     
        _ => panic!("Expression {:?} not found", expr)
    }
}

pub fn tokenize(program:&str) ->Vec<lexer::token::Token>{
    let mut tokens:Vec<lexer::token::Token> = vec![];
    let exprs: Vec<&str> = program.split(" ").collect();
    let mut line:usize = 1; 
    for expr in exprs.iter() {
        if expr == &"\\n"{
            line += 1;
        } else{
            tokens.push(grammar(expr, line));
        }
    }
    tokens.push(lexer::token::Token::new(lexer::token::TokenType::EOF, line));
    return tokens
}