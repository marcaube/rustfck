use crate::token::Token;

pub struct Lexer {
    instructions: Vec<u8>,
}

impl Lexer {
    pub fn new(instructions: Vec<u8>) -> Lexer {
        Lexer { instructions }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut program = vec![];

        for c in &self.instructions {
            let token = match *c as char {
                '>' => Token::Right,
                '<' => Token::Left,
                '+' => Token::Add(1),
                '-' => Token::Sub,
                '.' => Token::Output,
                ',' => Token::Input,
                '[' => Token::LoopStart,
                ']' => Token::LoopEnd,
                _ => Token::Other,
            };

            program.push(token);
        }

        program
    }
}
