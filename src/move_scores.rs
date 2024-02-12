use crate::tic_tac_toe::*;
use crate::Move;
use getset::{CopyGetters, Getters};
use std::fmt;

/// Scores for moves in a position.
#[derive(Debug, Getters, CopyGetters)]
pub struct MoveScores {
    /// score[m] = the score for move m:
    ///
    /// The sum of all scores should be 1.0.
    #[getset(get = "pub")]
    score: [f64; Move::N],
    /// order[m] = i implies that m is the move with the ith highest score:
    #[getset(get = "pub")]
    order: [usize; Move::N],
    /// move_at[i] = m implies that m is the move with the ith highest score:
    #[getset(get = "pub")]
    move_at: [Move; Move::N],
    /// If all moves are 0.
    #[getset(get_copy = "pub")]
    all_zero: bool,
    /// The number of times a score has been adjusted.
    #[getset(get_copy = "pub")]
    adjusted: u64,
}

impl MoveScores {
    /// Initialize all legal moves to `initial_score` and the other to 0.
    pub fn initial(pos: &State) -> Self {
        let mut res = Self {
            score: [0.0; Move::N],
            order: Default::default(),
            move_at: Default::default(),
            all_zero: true,
            adjusted: 1,
        };
        // Fill order and move_at with an initial order:
        for i in 0..Move::N {
            res.order[i] = i;
            res.move_at[i] = Move::from_usize(i);
        }
        // Fill in legal moves:
        let moves = pos.legal_moves();
        if moves.size() > 0 {
            res.all_zero = false;
        }
        for (i, m) in moves.iter().enumerate() {
            let m_i = m.to_usize();
            res.score[m_i] = 1.0 / moves.size() as f64;
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
        if self.all_zero() {
            return;
        }
        let mut score_sum = 0.0;
        let mut prev_score = f64::INFINITY;
        for i in 0..Move::N {
            let m = self.move_at[i];
            let m_i = m.to_usize();
            assert_eq!(self.order[m_i], i);
            assert!(prev_score >= self.score[m_i]);
            score_sum += self.score[m_i];
            prev_score = self.score[m_i];
        }
        let epsilon = f64::EPSILON * 1e4;
        assert!(1.0 - epsilon < score_sum && score_sum < 1.0 + epsilon);
    }

    /// Multiply the score for a move relative the other scores. The sum of the scores will still
    /// be 1.0.
    pub fn multiply(&mut self, m: Move, factor: f64) -> f64 {
        assert!(factor.is_finite());
        let m_i = m.to_usize();
        assert!(0.0 < self.score[m_i]);
        self.score[m_i] *= factor;
        let total_factor = 1.0 / self.score.iter().sum::<f64>();
        self.score.iter_mut().for_each(|s| *s *= total_factor);
        self.adjusted += 1;
        let mut i = self.order[m_i];
        if factor > 1.0 {
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
        } else {
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
        }
        self.validate();
        total_factor
    }
}

impl fmt::Display for MoveScores {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "adjusted {}s, ", self.adjusted)?;
        let mut numfmtr = numfmt::Formatter::new().precision(numfmt::Precision::Significance(3));
        for (i, m) in self.move_at().iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            let score = self.score()[m.to_usize()];
            if score == 0.0 {
                if i == 0 {
                    write!(f, "EMPTY")?;
                }
                break;
            }
            write!(f, "{m}: {}", numfmtr.fmt2(score))?;
        }
        Ok(())
    }
}
