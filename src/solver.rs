use crate::board::*;

fn scan_row(board: &mut Board<SolvingCell>, i: usize, j: usize, n: usize) -> bool {
    let mut has_update = false;
    for j2 in 0..N {
        if j != j2 {
            has_update |= board.0[i][j2].remove(n)
        }
    }
    has_update
}

fn scan_col(board: &mut Board<SolvingCell>, i: usize, j: usize, n: usize) -> bool {
    let mut has_update = false;
    for i2 in 0..N {
        if i != i2 {
            has_update |= board.0[i2][j].remove(n)
        }
    }
    has_update
}

fn scan_block(board: &mut Board<SolvingCell>, i: usize, j: usize, n: usize) -> bool {
    let top = i / 3 * 3;
    let bottom = top + 3;
    let left = j / 3 * 3;
    let right = left + 3;
    let mut has_update = false;
    for i2 in top..bottom {
        if i != i2 {
            for j2 in left..right {
                if j != j2 {
                    has_update |= board.0[i2][j2].remove(n)
                }
            }
        }
    }
    has_update
}

fn examine_cell(board: &mut Board<SolvingCell>, i: usize, j: usize) -> bool {
    if !board.0[i][j].has_update() {
        return false;
    }

    match board.0[i][j].get_unique() {
        None => false,
        Some(n) => scan_row(board, i, j, n) | scan_col(board, i, j, n) | scan_block(board, i, j, n),
    }
}

fn sweep(board: &mut Board<SolvingCell>) -> bool {
    let mut has_update = false;
    for i in 0..N {
        for j in 0..N {
            has_update |= examine_cell(board, i, j);
        }
    }
    has_update
}

fn solve<F>(mut board: Board<SolvingCell>, mut f: F)
where
    F: FnMut(Board<usize>),
{
    while sweep(&mut board) {}

    if let Some(solution) = board.to_solution() {
        f(solution);
        return;
    }

    todo!("case analysis");
}

pub fn for_each_solution<F>(problem: &Board<Option<usize>>, f: F)
where
    F: FnMut(Board<usize>),
{
    // Convert to Board<SolvingCell>
    let mut solving_board = Board([[SolvingCell::new(None); N]; N]);
    for i in 0..N {
        for j in 0..N {
            solving_board.0[i][j] = SolvingCell::new(problem.0[i][j]);
        }
    }

    solve(solving_board, f);
}
