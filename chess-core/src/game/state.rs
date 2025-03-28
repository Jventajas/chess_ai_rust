use std::fmt::Display;
use crate::game::bitboard::*;

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

    pub fn new_game() -> Self {
        let board = [
            Bitboard::from(WHITE_PAWNS),
            Bitboard::from(BLACK_PAWNS),
            Bitboard::from(WHITE_ROOKS),
            Bitboard::from(BLACK_ROOKS),
            Bitboard::from(WHITE_KNIGHTS),
            Bitboard::from(BLACK_KNIGHTS),
            Bitboard::from(WHITE_BISHOPS),
            Bitboard::from(BLACK_BISHOPS),
            Bitboard::from(WHITE_QUEEN),
            Bitboard::from(BLACK_QUEEN),
            Bitboard::from(WHITE_KING),
            Bitboard::from(BLACK_KING),
        ];

        Self {
            board,
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


    #[test]
    fn test_gamestate_new_game_display() {
        let game_state = GameState::new_game();
        let display_output = format!("{}", game_state);
        for (i, bitboard) in game_state.board.iter().enumerate() {
            let expected_line = format!("{}", bitboard);
            assert!(
                display_output.contains(&expected_line),
                "Bitboard at position {} was not displayed correctly in the output",
                i
            );
        }
    }
}



