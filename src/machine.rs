use crate::*;
use getset::Getters;
use rand::prelude::*;
use std::collections::HashMap;

const DECISIVE_FACTOR: f64 = 32.0;
const DRAW_FACTOR: f64 = 0.9;

const RAND_SEED: u64 = 42;

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
            .or_insert_with(|| MoveScores::initial(&pos))
    }

    /// Select a move for a position.
    pub fn select_move(&mut self, pos: State) -> Option<Move> {
        let moves = self
            .values
            .entry(pos)
            .or_insert_with(|| MoveScores::initial(&pos));
        if moves.all_zero() {
            return None;
        }
        let mut x = self.rng.gen::<f64>();
        let mut i = Move::N - 1;
        loop {
            let m = moves.move_at()[i];
            let m_score = moves.score()[m.to_usize()];
            if m_score == 0.0 {
                i -= 1;
                continue;
            }
            if x < m_score {
                break Some(m);
            }
            x -= m_score;
            assert_ne!(i, 0);
            i -= 1;
        }
    }

    /// Let the machine play a training match against itself and update scores accordingly.
    pub fn play_training_match(&mut self) -> GameResult {
        let mut pos = State::new();

        // moves[p] is the moves played by player p:
        let mut moves = [Vec::new(), Vec::new()];
        let mut turn = Crosses;
        let result = loop {
            let Some(m) = self.select_move(pos) else {
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
        let factors: [(Player, f64); 2] = match result {
            Draw => [(Crosses, DRAW_FACTOR), (Naughts, DRAW_FACTOR)],
            Win { winner, .. } => [
                (winner, DECISIVE_FACTOR),
                (winner.opponent(), 1.0 / DECISIVE_FACTOR),
            ],
        };
        for (player, mut factor) in factors {
            for (pos, m) in moves[player as usize].iter().rev() {
                factor = self
                    .values
                    .get_mut(pos)
                    .unwrap()
                    .multiply(*m, factor)
                    .cbrt();
            }
        }
        result
    }
}
