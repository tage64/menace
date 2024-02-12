use anyhow::Context as _;
use std::fmt;
use std::str::FromStr;

/// A move.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[repr(transparent)]
pub struct Move(usize);

/// A set of moves.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[repr(transparent)]
pub struct MoveSet(usize);

impl Move {
    pub const N: usize = 9;

    /// Get a list of all moves.  Garanteed to be ordered by `Move::to_usize()`.
    pub fn all() -> impl Iterator<Item = Self> {
        (0..Self::N).map(|i| Move(i))
    }

    pub const fn to_usize(&self) -> usize {
        self.0
    }

    pub const fn from_usize(i: usize) -> Self {
        Move(i)
    }

    pub const fn to_move_set(self) -> MoveSet {
        let mut set = MoveSet::empty();
        set = set.add(self);
        set
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row = self.0 / 3;
        let col = self.0 % 3;
        write!(
            f,
            "{}{}",
            char::from_u32('a' as u32 + row as u32).unwrap(),
            col + 1
        )
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut chars = s.chars();
        let letter = chars.next().context("A move cannot be an empty string.")?;
        let letter_idx = letter as u32;
        let a_idx = 'a' as u32;
        anyhow::ensure!(
            a_idx <= letter_idx && letter_idx < a_idx + 3,
            "The letter in a move must be between a and c."
        );
        let row = letter_idx - a_idx;
        let col = chars
            .as_str()
            .parse::<usize>()
            .context("The move column should be represented by a number.")?;
        anyhow::ensure!(
            1 <= col && col < 4,
            "The move column must be in the range [1, 3]."
        );
        let col = col - 1;
        let m_i = row as usize * 3 + col;
        assert!(m_i < Move::N);
        Ok(Move::from_usize(m_i))
    }
}

impl MoveSet {
    pub const fn empty() -> Self {
        MoveSet(0)
    }

    pub const fn all() -> Self {
        MoveSet(usize::MAX)
    }

    pub fn from_fn(f: impl FnMut(Move) -> bool) -> Self {
        Self::all().filter(f)
    }

    pub fn contains(self, m: Move) -> bool {
        self.0 & (1usize << m.to_usize()) != 0
    }

    #[must_use]
    pub const fn add(self, m: Move) -> Self {
        MoveSet(self.0 | (1usize << m.to_usize()))
    }

    #[must_use]
    pub const fn remove(self, m: Move) -> Self {
        MoveSet(self.0 & !(1usize << m.to_usize()))
    }

    pub fn iter(self) -> impl Iterator<Item = Move> {
        (0..Move::N).filter_map(move |i| {
            if self.0 & (1usize << i) != 0 {
                Some(Move::from_usize(i))
            } else {
                None
            }
        })
    }

    #[must_use]
    pub fn filter(mut self, mut f: impl FnMut(Move) -> bool) -> Self {
        for m in self.iter() {
            if !f(m) {
                self = self.remove(m);
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_set() {
        // Test empty and all:
        assert_eq!(MoveSet::empty().iter().count(), 0);
        assert_eq!(MoveSet::all().iter().count(), Move::N);
        let mut mset = MoveSet::all();
        for m in (0..Move::N).map(Move::from_usize) {
            assert!(mset.contains(m));
        }

        // Test remove and add:
        let m = Move::from_usize(4);
        assert!(mset.contains(m));
        mset = mset.remove(m);
        assert!(!mset.contains(m));
        mset = mset.add(m);
        assert!(mset.contains(m));

        // Test from_fn and filter:
        mset = MoveSet::from_fn(|m| m.to_usize() % 2 == 0);
        assert!(mset.iter().all(|m| m.to_usize() % 2 == 0));
        mset = mset.filter(|m| m.to_usize() % 3 == 0);
        assert!(mset.iter().all(|m| m.to_usize() % 2 == 0));
        assert_eq!(mset.iter().count(), 1 + Move::N / 6);
    }
}
