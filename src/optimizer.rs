use crate::token::Token;

pub struct Optimizer {
    tokens: Vec<Token>,
}

impl Optimizer {
    pub fn new(tokens: Vec<Token>) -> Optimizer {
        Optimizer { tokens }
    }

    pub fn optimize(&mut self) -> Vec<Token> {
        self.optimize_repeated_opcodes();

        self.tokens
    }

    pub fn optimize_repeated_opcodes(&mut self) {
        let optimized_tokens = vec![];

        for token in &self.tokens {
            match token.clone() {
                Token::Add(i) => {
                    let count = i;

                    optimized_tokens.push(Token::Add(count.to_owned()));
                }
                _ => optimized_tokens.push(token),
            }
        }
    }
}
