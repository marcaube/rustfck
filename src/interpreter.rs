use std::io::{self, Read, Write};

use crate::token::Token;

pub struct Interpreter {
    instructions: Vec<Token>,
    instruction_pointer: usize,
    data_pointer: usize,
    memory: Vec<u8>,
}

impl Interpreter {
    pub fn new(instructions: Vec<Token>) -> Interpreter {
        Interpreter {
            instructions,
            instruction_pointer: 0,
            data_pointer: 0,
            memory: vec![0u8; 30_000],
        }
    }

    pub fn eval(&mut self) {
        while self.instruction_pointer < self.instructions.len() {
            // See: https://en.wikipedia.org/wiki/Brainfuck#Commands
            match self.instructions[self.instruction_pointer] {
                // Increment the data pointer (to point to the next cell to the right).
                Token::Right => self.data_pointer += 1,

                // Decrement the data pointer (to point to the next cell to the left).
                Token::Left => self.data_pointer -= 1,

                // Increment (increase by one) the byte at the data pointer.
                Token::Add => {
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1)
                }

                // Decrement (decrease by one) the byte at the data pointer.
                Token::Sub => {
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_sub(1)
                }

                // Output the byte at the data pointer.
                Token::Output => io::stdout()
                    .write_all(&self.memory[self.data_pointer..self.data_pointer + 1])
                    .unwrap(),

                // Accept one byte of input, storing its value in the byte at the data pointer.
                Token::Input => io::stdin()
                    .read_exact(&mut self.memory[self.data_pointer..self.data_pointer + 1])
                    .unwrap(),

                // If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the
                // next command, jump it forward to the command after the matching ] command.
                Token::LoopStart if self.memory[self.data_pointer] == 0 => {
                    let mut count = 1;

                    while count > 0 {
                        self.instruction_pointer += 1;

                        if self.instructions[self.instruction_pointer] == Token::LoopEnd {
                            count -= 1
                        } else if self.instructions[self.instruction_pointer] == Token::LoopStart {
                            count += 1
                        }
                    }
                }

                // If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to
                // the next command, jump it back to the command after the matching [ command.
                Token::LoopEnd if self.memory[self.data_pointer] != 0 => {
                    let mut count = 1;

                    while count > 0 {
                        self.instruction_pointer -= 1;

                        if self.instructions[self.instruction_pointer] == Token::LoopStart {
                            count -= 1
                        } else if self.instructions[self.instruction_pointer] == Token::LoopEnd {
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
