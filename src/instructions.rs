/**
 * An opcode is the first byte of an instruction in machine language which tells
 *  the hardware what operation needs to be performed with this instruction
 */
#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,        // HALT
    LOAD,       // Load variable into register
    ADD,        // Add
    SUB,        // Subtract
    MUL,        // Multiply
    DIV,        // Divide
    JMP,        // Absolute Jump (changes the program counter relative to the VM)
    JMPF,       // Relative Jump Forward (changes the program counter relative to the current instruction)
    JMPB,       // Relative Jump backward
    EQ,         // Equal
    NEQ,        // Not Equal
    GT,         // Greater Then
    LT,         // Less Then
    GQT,        // Greater or equal to
    LQT,        // Less then or equal to
    JEQ,        // Jump if equal to
    JNEQ,       // Jump if not equal to
    IGL,        // Illegal opcode
}


#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,

}

impl Instruction {
    fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode
        }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::GT,
            11 => Opcode::LT,
            12 => Opcode::GQT,
            13 => Opcode::LQT,
            14 => Opcode::JEQ,
            15 => Opcode::JNEQ,
            _ => Opcode::IGL,
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);

        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}