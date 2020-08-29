use proconio::input;

const fn add(a: i32, b: i32) -> i32 {
    a + b
}

// const fn add(a: i32, b: i32) -> i32 {
    // if a > 5 {
        // a + b
    // } else {
        // b + a
    // }
// }

use std::io::{stdout, Write};
macro_rules! flush {
    () => {
        stdout().flush().unwrap();
    };
}

fn main() {
    print!("Input: ");
    flush!();
    input! {
        x: i32,
        y: i32,
    }
    println!("{}", add(x, y));
}
