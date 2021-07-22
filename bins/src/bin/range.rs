// macro_rules! range {
//     ($stop:expr) => {
//         0..$stop
//     };
//     ($start:expr, $stop:expr) => {
//         $start..$stop
//     };
//     ($start:expr, $stop:expr, -$step:expr) => {
//         ($stop + 1..$start + 1).rev().step_by($step)
//     };
//     ($start:expr, $stop:expr, $step:expr) => {
//         ($start..$stop).step_by($step)
//     };
// }

macro_rules! range {
    ($stop:expr) => {
        0..$stop
    };
    ($start:expr, $stop:expr) => {
        $start..$stop
    };
    ($start:expr, $stop:expr, $step:expr) => {{
        let step: i64 = $step as i64;
        #[allow(clippy::reversed_empty_ranges)]
        if $step < 0 {
            ($stop + 1..$start + 1)
                .rev()
                .step_by(-step as usize)
                .collect::<Vec<_>>()
                .into_iter()
        } else {
            ($start..$stop)
                .step_by(step as usize)
                .collect::<Vec<_>>()
                .into_iter()
        }
    }};
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
    let x = -5;
    for i in range!(20, 2, x) {
        println!("E: {}", i);
    }
}
