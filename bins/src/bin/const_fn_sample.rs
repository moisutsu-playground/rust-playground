const fn cul(a: i32, b: i32) -> i32 {
    a + b
}

const X: i32 = 10;
const Y: i32 = 21;

fn main() {
    const RESULT: i32 = cul(X, Y);
    println!("{}", RESULT);
}
