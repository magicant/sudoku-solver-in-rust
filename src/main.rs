mod board;
mod solver;

use board::*;
use solver::for_each_solution;
use std::io::BufRead;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

fn read_problem() -> Result<Board<Option<usize>>> {
    let mut board = Board([[None; N]; N]);
    for (i, line) in std::io::stdin().lock().lines().take(N).enumerate() {
        let line = line?;
        for (j, n) in line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .take(N)
            .enumerate()
        {
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
