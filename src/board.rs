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

    /// Returns the number if `self` is unique.
    pub fn get_unique(&self) -> Option<usize> {
        let mut u = None;
        for (n, &b) in self.0.iter().enumerate() {
            if b {
                match u {
                    None => u = Some(n),
                    Some(_) => return None
                }
            }
        }
        u
    }

    /// Tests if `self` is unique.
    pub fn is_unique(&self) -> bool {
        matches!(self.get_unique(), Some(_))
    }

    /// Tests if `self` is empty.
    pub fn is_empty(&self) -> bool {
        !self.0.iter().any(|&b| b)
    }

    /// Iterates possibilities.
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.0.iter().enumerate()
            .filter_map(|(n, &b)| if b { Some(n) } else { None })
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

    #[test]
    fn get_unique() {
        assert_eq!(PossibilitySet::EMPTY.get_unique(), None);
        assert_eq!(PossibilitySet::FULL.get_unique(), None);
        assert_eq!(PossibilitySet::unique(2).get_unique(), Some(2));
        assert_eq!(PossibilitySet::unique(4).get_unique(), Some(4));
    }

    #[test]
    fn is_unique() {
        assert!(!PossibilitySet::EMPTY.is_unique());
        assert!(!PossibilitySet::FULL.is_unique());
        assert!(PossibilitySet::unique(3).is_unique());
        assert!(PossibilitySet::unique(6).is_unique());
        assert!(!PossibilitySet([
            false, true, true, false, true, true, false, true, false])
                .is_unique());
    }

    #[test]
    fn is_empty() {
        assert!(PossibilitySet::EMPTY.is_empty());
        assert!(!PossibilitySet::FULL.is_empty());
        assert!(!PossibilitySet::unique(3).is_empty());
        assert!(!PossibilitySet::unique(6).is_empty());
        assert!(!PossibilitySet([
            false, true, true, false, true, true, false, true, false])
                .is_empty());
    }

    #[test]
    fn possibility_iter() {
        assert_eq!(PossibilitySet::EMPTY.iter().next(), None);
        assert_eq!(PossibilitySet::FULL.iter().collect::<Vec<usize>>(),
            (0..N).collect::<Vec<usize>>());
        assert_eq!(PossibilitySet([
            true, false, true, true, false, false, true, false, true]).iter()
            .collect::<Vec<usize>>(),
            vec![0, 2, 3, 6, 8]);
    }

}
