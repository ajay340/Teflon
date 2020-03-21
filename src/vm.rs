use crate::instructions::Opcode;

pub struct VM {
    registers: [i32; 32],   // Use an array because we know the size at compile time 
    pc: usize,              // The program counter
    program: Vec<u8>,       // A vector to store the program bytecode
    remainder: u32,         // The remainder of dividing two numbers
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],  // initialize all registers to 0
            pc: 0,
            program: vec![],
            remainder: 0
        }
    }

    // Loops as long as there are still instructions available
    pub fn run(&mut self) {
        let mut is_done: bool = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    // Executes only one instruction. Meant for debugging the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    // Executes the next instruction that is read from the program
    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number: u16 = self.next_16_bits();
                self.registers[register] = number as i32;
            },
            Opcode::ADD => {
                // Value from first register
                let register1 = self.registers[self.next_8_bits() as usize];
                // Value from second register
                let register2 = self.registers[self.next_8_bits() as usize];
                // Place the new value in the specified register
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            Opcode::SUB => {
                // Value from first register
                let register1 = self.registers[self.next_8_bits() as usize];
                // Value from second register
                let register2 = self.registers[self.next_8_bits() as usize];
                // Place the new value in the specified register
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            },
            Opcode::MUL => {
                // Value from first register
                let register1 = self.registers[self.next_8_bits() as usize];
                // Value from second register
                let register2 = self.registers[self.next_8_bits() as usize];
                // Place the new value in the specified register
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 /register2;
                self.remainder = (register1 % register2) as u32;

            },
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            },
            _ => {
                println!("Unrecognized opcode found! Terminating!");
                return true;
            },
        }
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0],0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244]; // this is how we represent 500 using two u8s in little endian format
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_load_opcode2() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 0, 15]; // this is how we represent 500 using two u8s in little endian format
        test_vm.run();
        assert_eq!(test_vm.registers[0], 15);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        // 0-3: load 15 into register 0
        // 4-7: Load 5 into register 1
        // 7-10: Add register 0 and 1 and load into register 3
        test_vm.program = vec![1, 0, 0, 15, 1, 1, 0, 5, 2, 0, 1, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 20);
    }

    #[test]
    fn test_subtract_opcode() {
        let mut test_vm = VM::new();
        // 0-3: load 15 into register 0
        // 4-7: Load 5 into register 1
        // 7-10: Subtract register 0 and 1 and load into register 3
        test_vm.program = vec![1, 0, 0, 15, 1, 1, 0, 5, 3, 0, 1, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 10);
    }

    #[test]
    fn test_subtract_opcode_negative_number() {
        let mut test_vm = VM::new();
        // 0-3: load 5 into register 0
        // 4-7: Load 15 into register 1
        // 7-10: Subtract register 0 and 1 and load into register 3
        test_vm.program = vec![1, 0, 0, 5, 1, 1, 0, 15, 3, 0, 1, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3], -10);
    }

    #[test]
    fn test_multiply_opcode() {
        let mut test_vm = VM::new();
        // 0-3: load 15 into register 0
        // 4-7: Load 5 into register 1
        // 7-10: Subtract register 0 and 1 and load into register 3
        test_vm.program = vec![1, 0, 0, 15, 1, 1, 0, 5, 4, 0, 1, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 75);
    }

    #[test]
    fn test_divide_opcode_ans_as_whole_number() {
        let mut test_vm = VM::new();
        // 0-3: load 10 into register 0
        // 4-7: Load 2 into register 1
        // 7-10: divide register 0 and 1 and load into register 3
        test_vm.program = vec![1, 0, 0, 10, 1, 1, 0, 2, 5, 0, 1, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 5);
        assert_eq!(test_vm.remainder, 0);
    }

    #[test]
    fn test_divide_opcode_ans_as_float() {
        let mut test_vm = VM::new();
        // 0-3: load 15 into register 0
        // 4-7: Load 2 into register 1
        // 7-10: Subtract register 0 and 1 and load into register 3
        test_vm.program = vec![1, 0, 0, 15, 1, 1, 0, 2, 5, 0, 1, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 7);
        assert_eq!(test_vm.remainder, 1);
    }
}