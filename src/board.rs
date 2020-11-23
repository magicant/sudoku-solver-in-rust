/// The size (the length of a edge) of a board: 9.
pub const N: usize = 9;

/// Collection of possible numbers that may fill a cell.
pub struct PossibilitySet(pub [bool; N]);

impl PossibilitySet {

    /// The empty set.
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

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn unique_out_of_range() {
        let _ = PossibilitySet::unique(N);
    }

    #[test]
    fn count() {
        assert_eq!(PossibilitySet::EMPTY.count(), 0);
        assert_eq!(PossibilitySet::FULL.count(), N);
        assert_eq!(PossibilitySet::unique(3).count(), 1);
        assert_eq!(PossibilitySet([
            false, true, true, false, true, true, false, true, false]).count(),
            5);
    }

}
