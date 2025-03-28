use std::fmt::Display;
use crate::game::bitboard::Bitboard;

pub struct GameState {
    pub board: [Bitboard; 12],
    pub turn_white: bool,
}

impl GameState {

    pub fn new() -> Self {
        Self {
            board: [Bitboard::new(); 12],
            turn_white: true,
        }
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Turn: {}", if self.turn_white { "White" } else { "Black" })?;
        writeln!(f, "Board:")?;
        for bitboard in self.board.iter() {
            writeln!(f, "  {}", bitboard)?;
        }
        Ok( ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamestate_initial_bitboards_zero() {
        let game_state = GameState::new();
        for (i, bitboard) in game_state.board.iter().enumerate() {
            assert_eq!(bitboard.data, 0, "Bitboard at position {} was not initialized to zero", i);
        }
    }

    #[test]
    fn test_gamestate_initial_turn_white() {
        let game_state = GameState::new();
        assert!(game_state.turn_white, "It should be white's turn after game state initialization");
    }
}



