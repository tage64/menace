use crate::*;
use getset::Getters;
use rand::prelude::*;
use std::collections::HashMap;

/// Initial score for a legal move.
const INITIAL_SCORE: u32 = 4;
const GAMMA: f32 = 1.0;

const RAND_SEED: u64 = 43;

/// The machine playing tic-tac-toe.
#[derive(Debug, Getters)]
pub struct Machine {
    #[getset(get = "pub")]
    values: HashMap<State, MoveScores>,
    rng: StdRng,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            rng: StdRng::seed_from_u64(RAND_SEED),
        }
    }

    /// Get the move scores for a position.
    pub fn get_move_scores(&mut self, pos: State) -> &MoveScores {
        self.values
            .entry(pos)
            .or_insert_with(|| MoveScores::initial(&pos, INITIAL_SCORE))
    }

    /// Select a move for a position.
    pub fn select_move(&mut self, pos: State, verbose: bool) -> Option<Move> {
        let moves = self
            .values
            .entry(pos)
            .or_insert_with(|| MoveScores::initial(&pos, INITIAL_SCORE));
        if moves.score_sum() == 0 {
            return None;
        }
        let mut x = self.rng.gen_range(0..moves.score_sum());
        let mut i = 0;
        loop {
            let m = moves.move_at()[i];
            let m_score = moves.score()[m.to_usize()];
            if verbose {
                dbg!(x);
                dbg!(m_score);
            }
            if x < m_score {
                break Some(m);
            }
            x -= m_score;
            i += 1;
        }
    }

    /// Let the machine play a training match against itself and update scores accordingly.
    pub fn play_training_match(&mut self) -> GameResult {
        let mut pos = State::new();

        // moves[p] is the moves played by player p:
        let mut moves = [Vec::new(), Vec::new()];
        let mut turn = Crosses;
        let result = loop {
            let Some(m) = self.select_move(pos, false) else {
                break Win {
                    winner: turn.opponent(),
                    reason: Resignation,
                };
            };
            moves[turn as usize].push((pos, m));
            pos.play(m, turn);
            if let Some(res) = pos.result(turn) {
                break res;
            }
            turn = turn.opponent();
        };

        // Update scores.
        if let Win { winner, .. } = result {
            let looser = winner.opponent();
            let mut k = 1.0f32;
            for (pos, m) in moves[winner as usize].iter() {
                self.values
                    .get_mut(pos)
                    .unwrap()
                    .increase(*m, k.round() as u32);
                k *= GAMMA;
            }
            let mut k = 1.0f32;
            for (pos, m) in moves[looser as usize].iter() {
                self.values
                    .get_mut(pos)
                    .unwrap()
                    .decrease(*m, k.round() as u32);
                k *= GAMMA;
            }
        }
        result
    }
}
