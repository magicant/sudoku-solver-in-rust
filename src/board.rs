use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

/// The size (the length of a edge) of a block: 3.
pub const N_BLOCK: usize = 3;

/// The size (the length of a edge) of a board: 9.
pub const N: usize = N_BLOCK * N_BLOCK;

/// Collection of possible numbers that may fill a cell.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PossibilitySet(pub [bool; N]);

impl PossibilitySet {
    /// The empty set.
    #[cfg(test)]
    pub const EMPTY: PossibilitySet = PossibilitySet([false; N]);

    /// The set containing all possibilities.
    pub const FULL: PossibilitySet = PossibilitySet([true; N]);

    /// Returns a set containing only one possibility.
    ///
    /// # Panics
    ///
    /// If `n >= N`.
    pub fn unique(n: usize) -> PossibilitySet {
        let mut ns = [false; N];
        ns[n] = true;
        PossibilitySet(ns)
    }

    /// Number of possibilities in this set.
    pub fn count(&self) -> usize {
        self.0.iter().filter(|&b| *b).count()
    }

    /// Returns the number if `self` is unique.
    pub fn get_unique(&self) -> Option<usize> {
        let mut u = None;
        for (n, &b) in self.0.iter().enumerate() {
            if b {
                match u {
                    None => u = Some(n),
                    Some(_) => return None,
                }
            }
        }
        u
    }

    /// Iterates possibilities.
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(n, &b)| if b { Some(n) } else { None })
    }
}

/// Cell of an intermediate board used in solving.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SolvingCell {
    /// Possible values for this cell.
    value: PossibilitySet,
    /// Whether this cell's value has changed and elimination is pending.
    update: bool,
    /// Whether this cell has been found unique.
    unique: bool,
}

impl SolvingCell {
    /// Creates a new cell.
    pub fn new(v: Option<usize>) -> SolvingCell {
        match v {
            None => SolvingCell {
                value: PossibilitySet::FULL,
                update: false,
                unique: false,
            },
            Some(n) => SolvingCell {
                value: PossibilitySet::unique(n),
                update: true,
                unique: true,
            },
        }
    }

    /// Whether this cell's value has changed and elimination is pending.
    pub fn has_update(&self) -> bool {
        self.update
    }

    /// Clears update status.
    pub fn acknowledge(&mut self) {
        self.update = false;
    }

    /// Whether this cell has possibility to be `n` in the solution.
    pub fn can_be(&self, n: usize) -> bool {
        self.value.0[n]
    }

    /// Returns the number if `self` is unique.
    pub fn get_unique(&self) -> Option<usize> {
        self.value.get_unique()
    }

    /// Number of possibilities in this cell.
    pub fn count(&self) -> usize {
        self.value.count()
    }

    /// Iterates possibilities.
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.value.iter()
    }

    /// Remove the given possibility.
    /// Returns true if `n` was previously contained in `self`.
    pub fn remove(&mut self, n: usize) -> bool {
        self.value.0[n] && {
            self.value.0[n] = false;
            self.update = true;
            true
        }
    }
}

/// 9x9 collection of cells.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Board<T>(pub [[T; N]; N]);

impl Board<SolvingCell> {
    /// Convert to a final board if `self` is a valid solution.
    pub fn to_solution(&self) -> Option<Board<usize>> {
        let mut solution = Board([[0; N]; N]);
        for i in 0..N {
            for j in 0..N {
                solution.0[i][j] = self.0[i][j].get_unique()?;
            }
        }
        Some(solution)
    }
}

impl Display for Board<usize> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for line in &self.0 {
            let mut first = true;
            for cell in line {
                if first {
                    first = false;
                } else {
                    f.write_str(" ")?;
                }
                f.write_fmt(format_args!("{}", cell + 1))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

/// Iterator of cells in a row.
pub fn row_iter(i: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..N).map(move |j| (i, j))
}

/// Iterator of cells in a column.
pub fn col_iter(j: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..N).map(move |i| (i, j))
}

/// Iterator of cells in a block.
///
/// # Panics
///
/// `i` and `j` must be 0, 3 or 6; otherwise this function panics.
pub fn block_iter(i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    assert_eq!(i % N_BLOCK, 0);
    assert_eq!(j % N_BLOCK, 0);
    assert!(i / N_BLOCK < N_BLOCK);
    assert!(j / N_BLOCK < N_BLOCK);
    (0..N).map(move |n| (i + n / N_BLOCK, j + n % N_BLOCK))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn possibility_set_unique_out_of_range() {
        let _ = PossibilitySet::unique(N);
    }

    #[test]
    fn possibility_set_count() {
        assert_eq!(PossibilitySet::EMPTY.count(), 0);
        assert_eq!(PossibilitySet::FULL.count(), N);
        assert_eq!(PossibilitySet::unique(3).count(), 1);
        assert_eq!(
            PossibilitySet([false, true, true, false, true, true, false, true, false]).count(),
            5
        );
    }

    #[test]
    fn possibility_set_get_unique() {
        assert_eq!(PossibilitySet::EMPTY.get_unique(), None);
        assert_eq!(PossibilitySet::FULL.get_unique(), None);
        assert_eq!(PossibilitySet::unique(2).get_unique(), Some(2));
        assert_eq!(PossibilitySet::unique(4).get_unique(), Some(4));
    }

    #[test]
    fn possibility_set_iter() {
        assert_eq!(PossibilitySet::EMPTY.iter().next(), None);
        assert_eq!(
            PossibilitySet::FULL.iter().collect::<Vec<usize>>(),
            (0..N).collect::<Vec<usize>>()
        );
        assert_eq!(
            PossibilitySet([true, false, true, true, false, false, true, false, true])
                .iter()
                .collect::<Vec<usize>>(),
            vec![0, 2, 3, 6, 8]
        );
    }

    #[test]
    fn solving_cell_new_none() {
        let none = SolvingCell::new(None);
        assert_eq!(none.count(), 9);
        assert!(!none.has_update());
        assert!(none.can_be(0));
        assert!(none.can_be(1));
        assert!(none.can_be(4));
        assert!(none.can_be(N - 2));
        assert!(none.can_be(N - 1));
    }

    #[test]
    fn solving_cell_new_some() {
        let some = SolvingCell::new(Some(4));
        assert_eq!(some.iter().collect::<Vec<usize>>(), vec![4]);
        assert!(some.has_update());
        assert!(!some.can_be(0));
        assert!(!some.can_be(1));
        assert!(some.can_be(4));
        assert!(!some.can_be(N - 2));
        assert!(!some.can_be(N - 1));
    }

    #[test]
    fn solving_cell_acknowledge() {
        let mut cell = SolvingCell::new(Some(6));
        cell.acknowledge();
        assert!(!cell.has_update());
        cell.acknowledge();
        assert!(!cell.has_update());
    }

    #[test]
    fn solving_cell_get_unique() {
        assert_eq!(SolvingCell::new(None).get_unique(), None);
        assert_eq!(SolvingCell::new(Some(1)).get_unique(), Some(1));
        assert_eq!(SolvingCell::new(Some(8)).get_unique(), Some(8));
    }

    #[test]
    fn row_iter_values() {
        assert_eq!(
            row_iter(3).collect::<Vec<_>>(),
            vec![
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (3, 5),
                (3, 6),
                (3, 7),
                (3, 8)
            ]
        );
    }

    #[test]
    fn col_iter_values() {
        assert_eq!(
            col_iter(7).collect::<Vec<_>>(),
            vec![
                (0, 7),
                (1, 7),
                (2, 7),
                (3, 7),
                (4, 7),
                (5, 7),
                (6, 7),
                (7, 7),
                (8, 7)
            ]
        );
    }

    #[test]
    fn block_iter_values() {
        assert_eq!(
            block_iter(0, 6).collect::<Vec<_>>(),
            vec![
                (0, 6),
                (0, 7),
                (0, 8),
                (1, 6),
                (1, 7),
                (1, 8),
                (2, 6),
                (2, 7),
                (2, 8)
            ]
        );
        assert_eq!(
            block_iter(6, 3).collect::<Vec<_>>(),
            vec![
                (6, 3),
                (6, 4),
                (6, 5),
                (7, 3),
                (7, 4),
                (7, 5),
                (8, 3),
                (8, 4),
                (8, 5)
            ]
        );
    }
}
