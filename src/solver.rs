use crate::board::*;

fn examine_row(board: &mut Board<SolvingCell>, i: usize) -> bool {
    let mut has_update = false;
    'n: for n in 0..N {
        // Find cells that can be n.
        let mut found_j = None;
        for j in 0..N {
            if board.0[i][j].can_be(n) {
                match found_j {
                    None => found_j = Some(j),
                    Some(_) => continue 'n,
                }
            }
        }

        // If there's exactly one such cell, make it unique.
        if let Some(j) = found_j {
            match board.0[i][j].get_unique() {
                None => {
                    board.0[i][j] = SolvingCell::new(Some(n));
                    has_update = true;
                }
                Some(n2) => debug_assert_eq!(n, n2),
            }
        }
    }
    has_update
}

fn examine_col(board: &mut Board<SolvingCell>, j: usize) -> bool {
    let mut has_update = false;
    'n: for n in 0..N {
        // Find cells that can be n.
        let mut found_i = None;
        for i in 0..N {
            if board.0[i][j].can_be(n) {
                match found_i {
                    None => found_i = Some(i),
                    Some(_) => continue 'n,
                }
            }
        }

        // If there's exactly one such cell, make it unique.
        if let Some(i) = found_i {
            match board.0[i][j].get_unique() {
                None => {
                    board.0[i][j] = SolvingCell::new(Some(n));
                    has_update = true;
                }
                Some(n2) => debug_assert_eq!(n, n2),
            }
        }
    }
    has_update
}

fn examine_block(board: &mut Board<SolvingCell>, i: usize, j: usize) -> bool {
    let mut has_update = false;
    'n: for n in 0..N {
        // Find cells that can be n.
        let mut found_cell = None;
        for i2 in i..(i + N_BLOCK) {
            for j2 in j..(j + N_BLOCK) {
                if board.0[i2][j2].can_be(n) {
                    match found_cell {
                        None => found_cell = Some((i2, j2)),
                        Some(_) => continue 'n,
                    }
                }
            }
        }

        // If there's exactly one such cell, make it unique.
        if let Some((i2, j2)) = found_cell {
            match board.0[i2][j2].get_unique() {
                None => {
                    board.0[i2][j2] = SolvingCell::new(Some(n));
                    has_update = true;
                }
                Some(n2) => debug_assert_eq!(n, n2),
            }
        }
    }
    has_update
}

fn filter_row(board: &mut Board<SolvingCell>, i: usize, j: usize, n: usize) -> bool {
    let mut has_update = false;
    for j2 in 0..N {
        if j != j2 {
            has_update |= board.0[i][j2].remove(n)
        }
    }
    has_update
}

fn filter_col(board: &mut Board<SolvingCell>, i: usize, j: usize, n: usize) -> bool {
    let mut has_update = false;
    for i2 in 0..N {
        if i != i2 {
            has_update |= board.0[i2][j].remove(n)
        }
    }
    has_update
}

fn filter_block(board: &mut Board<SolvingCell>, i: usize, j: usize, n: usize) -> bool {
    let top = i / N_BLOCK * N_BLOCK;
    let bottom = top + N_BLOCK;
    let left = j / N_BLOCK * N_BLOCK;
    let right = left + N_BLOCK;
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
        Some(n) => {
            filter_row(board, i, j, n) | filter_col(board, i, j, n) | filter_block(board, i, j, n)
        }
    }
}

fn sweep(board: &mut Board<SolvingCell>) -> bool {
    let mut has_update = false;

    for i in 0..N {
        has_update |= examine_row(board, i);
    }
    for j in 0..N {
        has_update |= examine_col(board, j);
    }
    for i in 0..N_BLOCK {
        for j in 0..N_BLOCK {
            has_update |= examine_block(board, i * N_BLOCK, j * N_BLOCK);
        }
    }

    for i in 0..N {
        for j in 0..N {
            has_update |= examine_cell(board, i, j);
        }
    }

    has_update
}

fn case_analysis<F>(board: Board<SolvingCell>, f: F)
where
    F: FnMut(Board<usize>) + Copy,
{
    // Find a cell with least possibilities.
    let k = (0..(N * N))
        .min_by_key(|k| {
            let c = board.0[k / N][k % N].count();
            if c == 1 {
                N + 1
            } else {
                c
            }
        })
        .unwrap();

    // Assume each possibility and solve again.
    for n in board.0[k / N][k % N].iter() {
        let mut board2 = board;
        board2.0[k / N][k % N] = SolvingCell::new(Some(n));
        assert_ne!(board, board2);
        solve(board2, f);
    }
}

fn solve<F>(mut board: Board<SolvingCell>, mut f: F)
where
    F: FnMut(Board<usize>) + Copy,
{
    while sweep(&mut board) {}

    if let Some(solution) = board.to_solution() {
        f(solution);
        return;
    }

    case_analysis(board, f);
}

pub fn for_each_solution<F>(problem: &Board<Option<usize>>, f: F)
where
    F: FnMut(Board<usize>) + Copy,
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
