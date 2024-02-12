use crate::tic_tac_toe::*;
use crate::Move;
use getset::{CopyGetters, Getters};
use std::cmp;
use std::fmt;

/// Scores for moves in a position.
#[derive(Debug, Getters, CopyGetters)]
pub struct MoveScores {
    /// score[m] = the score for move m:
    #[getset(get = "pub")]
    score: [u32; Move::N],
    /// order[m] = i implies that m is the move with the ith highest score:
    #[getset(get = "pub")]
    order: [usize; Move::N],
    /// move_at[i] = m implies that m is the move with the ith highest score:
    #[getset(get = "pub")]
    move_at: [Move; Move::N],
    /// The sum of all scores.
    #[getset(get_copy = "pub")]
    score_sum: u32,
}

impl MoveScores {
    /// Initialize all legal moves to `initial_score` and the other to 0.
    pub fn initial(pos: &State, initial_score: u32) -> Self {
        let mut res = Self {
            score: [0; Move::N],
            order: Default::default(),
            move_at: Default::default(),
            score_sum: 0,
        };
        // Fill order and move_at with an initial order:
        for i in 0..Move::N {
            res.order[i] = i;
            res.move_at[i] = Move::from_usize(i);
        }
        // Fill in legal moves:
        for (i, m) in pos.legal_moves().iter().enumerate() {
            let m_i = m.to_usize();
            res.score[m_i] = initial_score;
            res.score_sum += initial_score;
            assert!(i <= m_i);
            let m2 = res.move_at[i];
            res.move_at[m_i] = m2;
            res.order[m2.to_usize()] = m_i;
            res.move_at[i] = m;
            res.order[m_i] = i;
        }
        res.validate();
        res
    }

    /// Validate that all the internal arrays are correct.
    fn validate(&self) {
        let mut score_sum = 0;
        let mut prev_score = u32::MAX;
        for i in 0..Move::N {
            let m = self.move_at[i];
            let m_i = m.to_usize();
            assert_eq!(self.order[m_i], i);
            assert!(prev_score >= self.score[m_i]);
            score_sum += self.score[m_i];
            prev_score = self.score[m_i];
        }
        assert_eq!(score_sum, self.score_sum);
    }

    /// Increase the score for a move.
    pub fn increase(&mut self, m: Move, amount: u32) {
        let m_i = m.to_usize();
        assert_ne!(0, self.score[m_i]);
        self.score[m_i] += amount;
        self.score_sum += amount;
        let mut i = self.order[m_i];
        while i > 0 {
            let prev_m = self.move_at[i - 1];
            let prev_m_i = prev_m.to_usize();
            if self.score[prev_m_i] >= self.score[m_i] {
                break;
            }
            self.order[prev_m_i] += 1;
            self.order[m_i] -= 1;
            self.move_at[i] = prev_m;
            self.move_at[i - 1] = m;
            i -= 1;
        }
        self.validate();
    }

    /// Decrease the score for a move.
    pub fn decrease(&mut self, m: Move, amount: u32) {
        let m_i = m.to_usize();
        assert_ne!(0, self.score[m_i]);
        let amount = cmp::min(amount, self.score[m_i]);
        self.score[m_i] -= amount;
        self.score_sum -= amount;
        let mut i = self.order[m_i];
        while i + 1 < Move::N {
            let next_m = self.move_at[i + 1];
            let next_m_i = next_m.to_usize();
            if self.score[next_m_i] <= self.score[m_i] {
                break;
            }
            self.order[next_m_i] -= 1;
            self.order[m_i] += 1;
            self.move_at[i] = next_m;
            self.move_at[i + 1] = m;
            i += 1;
        }
        self.validate();
    }
}

impl fmt::Display for MoveScores {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "total: {}", self.score_sum())?;
        for m in self.move_at() {
            let score = self.score()[m.to_usize()];
            if score == 0 {
                break;
            }
            write!(f, ", {m}: {score}")?;
        }
        Ok(())
    }
}
