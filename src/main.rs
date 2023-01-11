use std::{
    env, error, fs,
    io::{self, Read, Write},
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let program = fs::read(env::args().nth(1).unwrap())?;

    let mut instruction_pointer = 0; // The current position in the program
    let mut tape = vec![0u8; 30000]; // The memory tape
    let mut data_pointer = 0; // Our "seek" postition on the tape

    while instruction_pointer < program.len() {
        // See: https://en.wikipedia.org/wiki/Brainfuck#Commands
        match program[instruction_pointer] as char {
            // Increment the data pointer (to point to the next cell to the right).
            '>' => data_pointer += 1,

            // Decrement the data pointer (to point to the next cell to the left).
            '<' => data_pointer -= 1,

            // Increment (increase by one) the byte at the data pointer.
            '+' => tape[data_pointer] = tape[data_pointer].wrapping_add(1),

            // Decrement (decrease by one) the byte at the data pointer.
            '-' => tape[data_pointer] = tape[data_pointer].wrapping_sub(1),

            // Output the byte at the data pointer.
            '.' => io::stdout().write_all(&tape[data_pointer..data_pointer + 1])?,

            // Accept one byte of input, storing its value in the byte at the data pointer.
            ',' => io::stdin().read_exact(&mut tape[data_pointer..data_pointer + 1])?,

            // If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the
            // next command, jump it forward to the command after the matching ] command.
            '[' if tape[data_pointer] == 0 => {
                let mut count = 1;

                while count > 0 {
                    instruction_pointer += 1;

                    if program[instruction_pointer] as char == ']' {
                        count -= 1
                    } else if program[instruction_pointer] as char == '[' {
                        count += 1
                    }
                }
            }

            // If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to
            // the next command, jump it back to the command after the matching [ command.
            ']' if tape[data_pointer] != 0 => {
                let mut count = 1;

                while count > 0 {
                    instruction_pointer -= 1;

                    if program[instruction_pointer] as char == '[' {
                        count -= 1
                    } else if program[instruction_pointer] as char == ']' {
                        count += 1
                    }
                }
            }

            // Ignore any other character that could be interspersed with the commands
            _ => (),
        }

        instruction_pointer += 1;
    }

    Ok(())
}
