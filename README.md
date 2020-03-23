# Teflon
A register based virtual machine built using Rust.

### About
This is a side project of mine that I am working on for fun. Currently I am following a [tutorial](https://blog.subnetzero.io/post/building-language-vm-part-01/) to familiarize myself with vm’s. I plan on adding my own features and optimizations at a later date.

### Current Feature being worked on
A an Assembler, more specifically a Lexer.

## Current Features
### Opcode
- HALT
- LOAD
- ADD
- SUB
- MULT
- DIV
- JMP (absolute)
- JMPF (relative)
- JMPB (relative)
- EQ
- GT
- LT
- GQT
- LQT
- JEQ
- JNEQ
- IGL

### REPL
- .history :: Shows all commands that were entered into the REPL.
- .program :: Lists all instructions that are currently loaded into the vm.
- .registers :: Shows the values that are currently in the vm registers
- .quit :: Quits the REPL
- hex code :: runs the given hex code in the vm 
    - EX: 01 01 03 E8 (loads 1000 into register 1)
