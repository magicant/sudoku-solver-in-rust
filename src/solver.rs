use crate::board::*;

fn examine_cell(board: &mut Board<SolvingCell>, i: usize, j: usize) {
    if !board.0[i][j].has_update() {
        return;
    }

    todo!();
}

fn solve<F>(mut board: Board<SolvingCell>, mut f: F)
where
    F: FnMut(Board<usize>),
{
    for i in 0..N {
        for j in 0..N {
            examine_cell(&mut board, i, j);
        }
    }

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
