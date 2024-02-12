use crate::Player;

/// The result of a game.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameResult {
    Draw,
    Win { winner: Player, reason: WinReason },
}
pub use GameResult::*;

/// A reason for winning a game.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WinReason {
    /// Three marks in a row or column.
    RowOrColumn,
    /// Three marks on the diagonal.
    Diagonal,
    /// Opponent resigned.
    Resignation,
}
pub use WinReason::*;
