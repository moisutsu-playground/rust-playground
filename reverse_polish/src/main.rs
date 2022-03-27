use std::io::{self, Write};

use reverse_polish::ReversePolish;

fn main() {
    let reverse_polish = ReversePolish::default();

    print!("Input: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let answer = reverse_polish.calculate(&input);

    println!("Answer: {}", answer);
}
