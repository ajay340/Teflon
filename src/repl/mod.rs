use crate::vm::VM;
use std::io;
use std::io::Write;
use std::num::ParseIntError;
use crate::lexer::Lexer;


pub struct REPL {
    command_buffer: Vec<String>,
    mode: Mode,
    vm: VM      // The vm that the repel will use to execute the code
}

// The mode that the VM is in
#[derive(PartialEq)]
enum Mode {
    Nil,
    Hex,
    Assembly,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],     // stores the previous commands
            vm: VM::new(),
            mode: Mode::Nil,
        }
    }

    fn switch_mode(&mut self, mode: Mode) {
        match mode {
            Mode::Hex => {
                self.mode = Mode::Hex;
                println!("Current mode is now: Hex");
            },
            Mode::Assembly  => {
                self.mode = Mode::Assembly;
                println!("Current mode is now: Assembly");
            },
            Mode::Nil => panic!("Invalid mode!")
        }
    }


    pub fn run(&mut self) {
        println!("Welcome to Teflon!");
        print!("Please choose the mode you wish to use:\n  1  -> HEX\n  2  -> Assembly\n");
        loop {
            // Allocate a new string to store whatever the user types at each iteration
            let mut buffer = String::new();

            // Blocking call until the user types a command
            let stdin = io::stdin();

            // Since print does not flush stdout we need to manually do it
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            // Read the string that the user typed
            stdin.read_line(&mut buffer).expect("Unable to read line from the user");
            let buf = buffer.trim();
            // store the command in the command buffer
            self.command_buffer.push(buf.to_string());
            
            // Determine what to do:
            if self.mode == Mode::Nil {
                match buf {
                    "1" => self.switch_mode(Mode::Hex),
                    "2" => self.switch_mode(Mode::Assembly),
                    _ => println!("Invalid choice. Please choose one of the following:\n  1  -> Hex\n  2  -> Assembly"),
                }
            } else {
                self.parse_input(buf);
            }
        }
    }


    fn parse_input(&mut self, buf: &str) {
        match buf {
            ".quit" => {
                println!("Good Bye! :)");
                std::process::exit(0);
            },
            ".history" => {
                for command in &self.command_buffer {
                    println!("{}", command);
                }
            },
            ".program" => {
                for instruction in &self.vm.program {
                    println!("{}", instruction);
                }
                println!("End of program listing");
            },
            ".registers" => {
                println!("Listing registers and all contents");
                println!("{:?}", self.vm.registers);
                println!("End of register listing");
            }
            ".mode" => {
                match self.mode {
                    Mode::Assembly => println!("Current mode is: Assembly"),
                    _ => println!("Current mode is: Hex"),
                }
            }
            _ => {
                match self.mode {
                    Mode::Assembly => self.assembly_mode(buf),
                    Mode::Hex => self.hex_mode(buf),
                    _ => panic!("Invalid REPL mode"),
                }
            }
        }
    }

    fn assembly_mode(&mut self, buf: &str) {
        let mut lexer = Lexer::new();
        lexer.lex_line(buf, 1);
        lexer.lex_debug();
    }

    fn hex_mode(&mut self, buf: &str) {
        let results = self.parse_hex(buf);
        match results {
            Ok(bytes) => {
                for byte in bytes {
                    self.vm.add_byte(byte)
                }
            },
            Err(_e) => {
                println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.");
            },
        };
        self.vm.run_once();
    }

    // Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    // Example for a LOAD command: 00 01 03 E8
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split: Vec<&str> = i.split(" ").collect();

        let mut results: Vec<u8> = vec![];

        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => results.push(result),
                Err(error) => return Err(error),
            }
        }

        Ok(results)
    }
}