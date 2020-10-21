use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let xs: Vec<i128> = (0..10000000).collect();
    let ys = xs.clone();

    let begin = Instant::now();
    let sum = xs.par_iter().map(|&x| x * x).sum::<i128>();
    println!("rayon = Sum: {}, Time: {:?}", sum, begin.elapsed());

    let begin = Instant::now();
    let sum = ys.par_iter().map(|&x| x * x).sum::<i128>();
    println!("normal = Sum: {}, Time: {:?}", sum, begin.elapsed());
}
