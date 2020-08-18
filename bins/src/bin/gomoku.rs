use std::fmt;
use std::io::{stdout, Write};

use anyhow::{anyhow, Result};
use proconio::input;

macro_rules! flush {
    () => {
        stdout().flush().unwrap();
    };
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
enum Goishi {
    Blank,
    Black,
    White,
}

#[derive(Debug)]
struct GomokuBoard {
    board: Vec<Vec<Goishi>>,
    edge_length: usize,
    ren_count: usize,
}

struct GomokuBoardBuilder {
    board: Vec<Vec<Goishi>>,
    edge_length: usize,
    ren_count: usize,
}

impl GomokuBoardBuilder {
    fn new() -> Self {
        let edge_length = 15;
        GomokuBoardBuilder {
            board: vec![vec![Goishi::Blank; edge_length]; edge_length],
            edge_length: edge_length,
            ren_count: 5,
        }
    }

    fn ren_count(&self, ren_count: usize) -> Self {
        GomokuBoardBuilder {
            board: self.board.clone(),
            edge_length: self.edge_length,
            ren_count: ren_count,
        }
    }

    fn edge_length(&self, edge_length: usize) -> Self {
        GomokuBoardBuilder {
            board: vec![vec![Goishi::Blank; edge_length]; edge_length],
            edge_length: edge_length,
            ren_count: self.ren_count,
        }
    }

    fn build(&self) -> GomokuBoard {
        GomokuBoard {
            board: self.board.clone(),
            edge_length: self.edge_length,
            ren_count: self.ren_count,
        }
    }
}

impl fmt::Display for GomokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_fmt = String::new();
        board_fmt.push_str("ーーーーーーー\n");
        for y in 0..self.edge_length {
            let edge_fmt = self.board[y]
                .iter()
                .map(|&goishi| match goishi {
                    Goishi::Blank => "　",
                    Goishi::White => "● ",
                    Goishi::Black => "○ ",
                })
                .collect::<Vec<&str>>()
                .join("｜");
            board_fmt.push_str(&format!("｜{}｜\n", edge_fmt));
            board_fmt.push_str("ーーーーーーー\n")
        }
        write!(f, "{}", board_fmt)
    }
}

impl GomokuBoard {
    fn set_goishi(&mut self, x: usize, y: usize, goishi: Goishi) -> Result<()> {
        if x >= self.edge_length || y >= self.edge_length {
            return Err(anyhow!("set goishi index out of range"));
        };
        if self.board[y][x] != Goishi::Blank {
            return Err(anyhow!("set goishi already goishi exist"));
        }
        self.board[y][x] = goishi;
        Ok(())
    }

    fn winner_check(&self) -> Option<Goishi> {
        for &goishi in vec![Goishi::Black, Goishi::White].iter() {
            for c in 0..self.edge_length {
                let mut row_count = 0;
                let mut column_count = 0;
                for r in 0..self.edge_length {
                    if self.board[c][r] == goishi {
                        row_count += 1;
                    } else {
                        row_count = 0;
                    }
                    if self.board[r][c] == goishi {
                        column_count += 1;
                    } else {
                        column_count = 0;
                    }
                    if row_count == self.ren_count || column_count == self.ren_count {
                        return Some(goishi);
                    }
                }
            }
        }
        None
    }
}

fn main() -> Result<()> {
    let mut board = GomokuBoardBuilder::new()
        .ren_count(3)
        .edge_length(3)
        .build();
    'outer: loop {
        for &goishi in vec![Goishi::Black, Goishi::White].iter() {
            println!("{}", board);
            print!("{:?}: ", goishi);
            flush!();
            input! {x: usize, y: usize};
            board.set_goishi(x, y, goishi)?;
            if let Some(winner) = board.winner_check() {
                println!("{}", board);
                println!("Winner: {:?}", winner);
                break 'outer;
            }
        }
    }
    Ok(())
}
