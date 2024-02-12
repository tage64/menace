use crate::*;
use std::convert::identity;
use std::fmt;
use std::mem::transmute;
use std::ops;

const CROSS_I: u8 = 0;
const NAUGHT_I: u8 = 1;

/// A mark in tic-tac-toe.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Mark {
    Cross = CROSS_I,
    Naught = NAUGHT_I,
    Blank,
}
pub use Mark::*;

/// A player in tic-tac-toe.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Player {
    Crosses = CROSS_I,
    Naughts = NAUGHT_I,
}
pub use Player::*;

/// A state in tic-tac-toe.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct State([[Mark; 3]; 3]);

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<Player> for Mark {
    fn from(p: Player) -> Mark {
        // Safety: Both Player and Mark are repr(u8) and Player is a subset of Mark.
        unsafe { transmute(p) }
    }
}

impl PartialEq<Player> for Mark {
    fn eq(&self, other: &Player) -> bool {
        Self::from(*other) == *self
    }
}

impl PartialEq<Mark> for Player {
    fn eq(&self, other: &Mark) -> bool {
        other == self
    }
}

impl Player {
    pub fn opponent(self) -> Self {
        match self {
            Self::Crosses => Self::Naughts,
            Self::Naughts => Self::Crosses,
        }
    }
}

impl ops::Index<Move> for State {
    type Output = Mark;
    fn index(&self, m: Move) -> &Self::Output {
        &self.0[m.to_usize() / 3][m.to_usize() % 3]
    }
}

impl ops::IndexMut<Move> for State {
    fn index_mut(&mut self, m: Move) -> &mut Self::Output {
        &mut self.0[m.to_usize() / 3][m.to_usize() % 3]
    }
}

impl State {
    /// An empty board.
    pub fn new() -> Self {
        State([[Blank; 3]; 3])
    }

    /// Check if the given player has three in a row.
    pub fn has_row(&self, player: Player) -> bool {
        (0..3)
            .map(|i| self.0[i].iter().all(|&m| m == player))
            .any(identity)
    }

    /// Check if the given player has three in a column.
    pub fn has_column(&self, player: Player) -> bool {
        (0..3)
            .map(|i| self.0.iter().map(|row| row[i] == player).all(identity))
            .any(identity)
    }

    /// Check if the given player has three on a diagonal.
    pub fn has_diagonal(&self, player: Player) -> bool {
        (0..3).map(|i| self.0[i][i]).all(|m| m == player)
            || (0..3).map(|i| self.0[i][2 - i]).all(|m| m == player)
    }

    /// Check if the game is a draw.
    pub fn is_draw(&self) -> bool {
        Move::all().all(|m| self[m] != Blank)
    }

    /// All legal moves in the position.  Garanteed to be ordered by `Move::to_usize()`.
    pub fn legal_moves(&self) -> MoveSet {
        MoveSet::from_fn(|m| self[m] == Blank)
    }

    /// Make a move with the given mark.
    pub fn play(&mut self, m: Move, player: Player) {
        debug_assert_eq!(self[m], Blank);
        self[m] = player.into();
    }

    /// Given the player who made the last move, return the result if the game is over.
    pub fn result(&self, player: Player) -> Option<GameResult> {
        if self.has_row(player) || self.has_column(player) {
            Some(Win {
                winner: player,
                reason: RowOrColumn,
            })
        } else if self.has_diagonal(player) {
            Some(Win {
                winner: player,
                reason: Diagonal,
            })
        } else if self.is_draw() {
            Some(Draw)
        } else {
            None
        }
    }
}
