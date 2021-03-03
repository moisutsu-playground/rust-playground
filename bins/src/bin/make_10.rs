use anyhow::Result;
use std::io::{stdin, stdout, Write};

macro_rules! flush {
    () => {
        stdout().flush().unwrap();
    };
}

fn main() -> Result<()> {
    print!("Input numbers: ");
    flush!();
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let input_numbers = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    for s in make_10(&input_numbers) {
        println!("{} = 10", s);
    }
    Ok(())
}

fn make_10(numbers: &Vec<i32>) -> Vec<String> {
    let calc_results = numbers.iter().map(|&n| n.to_string()).collect::<Vec<_>>();
    let numbers = numbers.iter().map(|&n| n as f64).collect();
    let r = _make_10(&numbers, &calc_results);
    r
}

fn _make_10(numbers: &Vec<f64>, calc_results: &Vec<String>) -> Vec<String> {
    let mut formulas_10 = vec![];
    if numbers.len() == 1 && numbers[0] == 10.0 {
        let mut formula = calc_results[0].clone();
        formula.remove(formula.len() - 1);
        formula.remove(0);
        formulas_10.push(formula);
        return formulas_10;
    }
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            for &op in ['+', '-', '/', '*'].iter() {
                let mut numbers = numbers.clone();
                let mut calc_results = calc_results.clone();
                let (num_i, num_j) = numbers.pop_two(i, j);
                let (res_i, res_j) = calc_results.pop_two(i, j);
                let mut each_results = match op {
                    '+' => {
                        numbers.push(num_i + num_j);
                        calc_results.push(format!("({} + {})", res_i, res_j));
                        _make_10(&numbers, &calc_results)
                    }
                    '-' => {
                        numbers.push(num_i - num_j);
                        calc_results.push(format!("({} - {})", res_i, res_j));
                        _make_10(&numbers, &calc_results)
                    }
                    '/' => {
                        numbers.push(num_i / num_j);
                        calc_results.push(format!("({} / {})", res_i, res_j));
                        _make_10(&numbers, &calc_results)
                    }
                    '*' => {
                        numbers.push(num_i * num_j);
                        calc_results.push(format!("({} * {})", res_i, res_j));
                        _make_10(&numbers, &calc_results)
                    }
                    _ => unreachable!(),
                };
                formulas_10.append(&mut each_results);
            }
        }
    }
    formulas_10
}

trait PopTwo {
    type Element;
    fn pop_two(&mut self, i: usize, j: usize) -> (Self::Element, Self::Element);
}

impl<T> PopTwo for Vec<T> {
    type Element = T;
    fn pop_two(&mut self, i: usize, j: usize) -> (Self::Element, Self::Element) {
        if i >= j {
            let item_i = self.swap_remove(i);
            let item_j = self.swap_remove(j);
            (item_i, item_j)
        } else {
            let item_j = self.swap_remove(j);
            let item_i = self.swap_remove(i);
            (item_i, item_j)
        }
    }
}
