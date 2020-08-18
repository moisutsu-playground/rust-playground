macro_rules! echo {
    ($x: expr) => {
        println!("{:?}", $x);
    };
}

fn main() {
    echo!(include_str!("list.rs").split(" "));
}
