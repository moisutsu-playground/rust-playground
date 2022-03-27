use crate::{Lexer, Token};

#[derive(Default)]
pub struct ReversePolish {
    lexer: Lexer,
}

impl ReversePolish {
    pub fn calculate(&self, input: &str) -> i64 {
        let tokens = self.lexer.tokenize(input);
        let mut stack = vec![];

        for token in tokens {
            match token {
                Token::Plus => {
                    let first = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(second + first);
                }
                Token::Minus => {
                    let first = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(second - first);
                }
                Token::Multi => {
                    let first = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(second * first);
                }
                Token::Div => {
                    let first = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(second / first);
                }
                Token::Num(n) => {
                    stack.push(n);
                }
            };
        }

        stack[0]
    }
}
