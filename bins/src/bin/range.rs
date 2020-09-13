macro_rules! range {
    ($stop:expr) => {
        0..$stop
    };
    ($start:expr, $stop:expr) => {
        $start..$stop
    };
    ($start:expr, $stop:expr, -$step:expr) => {
        ($stop + 1..$start + 1).rev().step_by($step)
    };
    ($start:expr, $stop:expr, $step:expr) => {
        ($start..$stop).step_by($step)
    };
}

fn main() {
    for i in range!(5) {
        println!("A: {}", i);
    }
    for i in range!(1, 5) {
        println!("B: {}", i);
    }
    for i in range!(1, 5, 2) {
        println!("C: {}", i);
    }
    for i in range!(3, 1, -1) {
        println!("D: {}", i);
    }
}
