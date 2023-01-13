use std::io::{self, Read, Write};

pub struct Interpreter {
    instructions: Vec<u8>,
    instruction_pointer: usize,
    data_pointer: usize,
    memory: Vec<u8>,
}

impl Interpreter {
    pub fn new(instructions: Vec<u8>) -> Interpreter {
        Interpreter {
            instructions,
            instruction_pointer: 0,
            data_pointer: 0,
            memory: vec![0u8; 30_000],
        }
    }

    pub fn run(&mut self) {
        while self.instruction_pointer < self.instructions.len() {
            // See: https://en.wikipedia.org/wiki/Brainfuck#Commands
            match self.instructions[self.instruction_pointer] as char {
                // Increment the data pointer (to point to the next cell to the right).
                '>' => self.data_pointer += 1,

                // Decrement the data pointer (to point to the next cell to the left).
                '<' => self.data_pointer -= 1,

                // Increment (increase by one) the byte at the data pointer.
                '+' => {
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1)
                }

                // Decrement (decrease by one) the byte at the data pointer.
                '-' => {
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_sub(1)
                }

                // Output the byte at the data pointer.
                '.' => io::stdout()
                    .write_all(&self.memory[self.data_pointer..self.data_pointer + 1])
                    .unwrap(),

                // Accept one byte of input, storing its value in the byte at the data pointer.
                ',' => io::stdin()
                    .read_exact(&mut self.memory[self.data_pointer..self.data_pointer + 1])
                    .unwrap(),

                // If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the
                // next command, jump it forward to the command after the matching ] command.
                '[' if self.memory[self.data_pointer] == 0 => {
                    let mut count = 1;

                    while count > 0 {
                        self.instruction_pointer += 1;

                        if self.instructions[self.instruction_pointer] as char == ']' {
                            count -= 1
                        } else if self.instructions[self.instruction_pointer] as char == '[' {
                            count += 1
                        }
                    }
                }

                // If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to
                // the next command, jump it back to the command after the matching [ command.
                ']' if self.memory[self.data_pointer] != 0 => {
                    let mut count = 1;

                    while count > 0 {
                        self.instruction_pointer -= 1;

                        if self.instructions[self.instruction_pointer] as char == '[' {
                            count -= 1
                        } else if self.instructions[self.instruction_pointer] as char == ']' {
                            count += 1
                        }
                    }
                }

                // Ignore any other character that could be interspersed with the commands
                _ => (),
            }

            self.instruction_pointer += 1;
        }
    }
}
