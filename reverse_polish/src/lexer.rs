pub enum Token {
    Num(i64),
    Plus,
    Minus,
    Multi,
    Div,
}

#[derive(Default)]
pub struct Lexer;

impl Lexer {
    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut tokens = vec![];

        for piece in input.split_whitespace() {
            let token = match piece {
                "+" => Token::Plus,
                "-" => Token::Minus,
                "*" => Token::Multi,
                "/" => Token::Div,
                p => Token::Num(p.parse().unwrap()),
            };
            tokens.push(token);
        }

        tokens
    }
}
