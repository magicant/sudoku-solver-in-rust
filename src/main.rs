mod board;
mod solver;

use board::*;
use solver::for_each_solution;
use std::io::stdin;
use std::io::BufRead;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

fn eof() -> Error {
    Error::new(ErrorKind::UnexpectedEof, "malformed problem")
}

fn read_problem() -> Result<Board<Option<usize>>> {
    let mut board = Board([[None; N]; N]);
    let input = stdin();
    let mut lines = input.lock().lines();
    for i in 0..N {
        let line = lines.next().unwrap_or_else(|| Err(eof()))?;
        let mut line = line.chars().filter_map(|c| c.to_digit(10));
        for j in 0..N {
            let n = line.next().ok_or_else(eof)?;
            board.0[i][j] = if n == 0 { None } else { Some((n - 1) as usize) }
        }
    }
    Ok(board)
}

fn main() -> Result<()> {
    let board = read_problem()?;
    let mut found_solution = false;

    for_each_solution(&board, |b| {
        found_solution = true;
        println!("{}", b);
    });

    if found_solution {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "no solution"))
    }
}
