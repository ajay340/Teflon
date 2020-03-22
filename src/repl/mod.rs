use crate::vm::VM;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM      // The vm that the repel will use to execute the code
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],     // stores the previous commands
            vm: VM::new(),
        }
    }


    pub fn run(&mut self) {
        println!("Welcome to Teflon!");
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
                _ => {
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
            }
        }
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