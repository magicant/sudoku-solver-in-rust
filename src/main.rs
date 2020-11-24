mod board;

use board::*;
use std::io::BufRead;
use std::io::Result;

fn read_board() -> Result<Board<Option<usize>>> {
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
    let _board = read_board()?;
    Ok(())
}
