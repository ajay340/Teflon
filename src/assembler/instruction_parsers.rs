use lexer::Token;

#[derive(debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token::Opcode,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

