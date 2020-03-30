/* Examples:

 1) LOAD $1 #10    => 01 01 00 0A


*/



/* A Program is composed of:
 -  a list of instructions 
*/
// struct Program {
//     instructions: Vec<Instruction>,
// }


/* A Instruction is composed of:
 - An Opcode 
 - A Register
 - A Integer Operand
 - A Newline
*/ 
// struct Instruction {
//     opcode: Opcode,
//     register: Register,
//     operand: IntegerOperand,
//     newline: char,
// }

/* A Register is composed of
 - The symbol $
 - A Number
 - A Space
*/
// struct Register {
//     number: Number,
// }

/* A IntegerOperand is composed og:
 - The symbol #
 - A Number
*/
// struct IntegerOperand {
//     number: Number,
// }



/* A number is composed of:
 - the symbols 0-9
    <digit> -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 
    <number> -> <digit> | <digit> <number>
*/
// struct Number {
//     number: u8,
// }


/* An opcode is composed of:
 - one or more letters in a row
 - A space
*/
// struct Opcode {
//     letters: String
// }

