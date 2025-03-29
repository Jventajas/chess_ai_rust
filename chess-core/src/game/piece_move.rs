use std::fmt::Display;
use crate::game::piece::Piece;

pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: Option<Piece>,
    pub capture: bool,
    pub en_passant: bool,
    pub kingside_castle: bool,
    pub queenside_castle: bool,
    pub check: bool,
    pub checkmate: bool,
    pub stalemate: bool,
    pub draw: bool,
}

impl Move {
    
    pub fn new(from: u8, to: u8) -> Move {
        Self {
            from,
            to,
            ..Default::default()
        }
    }

    pub fn new_all_fields(
        from: u8,
        to: u8,
        promotion: Option<Piece>,
        capture: bool,
        en_passant: bool,
        kingside_castle: bool,
        queenside_castle: bool,
        check: bool,
        checkmate: bool,
        stalemate: bool,
        draw: bool,
    ) -> Move {
        Self {
            from,
            to,
            promotion,
            capture,
            en_passant,
            kingside_castle,
            queenside_castle,
            check,
            checkmate,
            stalemate,
            draw,
        }
    }

    fn square_from(&self) -> String {
        let letter_idx = 65 + self.from % 8;
        let letter = letter_idx as char;
        let number = self.from / 8 + 1;
        format!("{}{}", letter, number)
    }

    fn square_to(&self) -> String {
        let letter_idx = 65 + self.to % 8;
        let letter = letter_idx as char;
        let number = self.to / 8 + 1;
        format!("{}{}", letter, number)
    }
}

impl Default for Move {
    fn default() -> Self {
        Move {
            from: 0,
            to: 0,
            promotion: None,
            capture: false,
            en_passant: false,
            kingside_castle: false,
            queenside_castle: false,
            check: false,
            checkmate: false,
            stalemate: false,
            draw: false,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut move_str = if self.kingside_castle {
            "O-O".to_string()
        } else if self.queenside_castle {
            "O-O-O".to_string()
        } else if self.en_passant {
            format!("{}x{} e.p.", self.square_from(), self.square_to())
        } else if let Some(promotion) = &self.promotion {
            if self.capture {
                format!("{}x{}={}", self.square_from(), self.square_to(), promotion)
            } else {
                format!("{}{}={}", self.square_from(), self.square_to(), promotion)
            }
        } else if self.capture {
            format!("{}x{}", self.square_from(), self.square_to())
        } else {
            format!("{}{}", self.square_from(), self.square_to())
        };

        if self.checkmate {
            move_str.push_str("#");
        } else if self.check {
            move_str.push_str("+");
        }

        write!(f, "{}", move_str)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::piece::Piece;

    #[test]
    fn test_simple_move() {
        let simple_move = Move::new(8, 16);
        assert_eq!(format!("{}", simple_move), "A2A3");
    }

    #[test]
    fn test_capture_move() {
        let capture_move = Move::new_all_fields(12, 20, None, true, false, false, false, false, false, false, false);
        assert_eq!(format!("{}", capture_move), "E2xE3");
    }

    #[test]
    fn test_en_passant_move() {
        let ep_move = Move::new_all_fields(24, 33, None, true, true, false, false, false, false, false, false);
        assert_eq!(format!("{}", ep_move), "A4xB5 e.p.");
    }

    #[test]
    fn test_kingside_castle_move() {
        let castle_move = Move::new_all_fields(4, 6, None, false, false, true, false, false, false, false, false);
        assert_eq!(format!("{}", castle_move), "O-O");
    }

    #[test]
    fn test_queenside_castle_move() {
        let castle_move = Move::new_all_fields(4, 2, None, false, false, false, true, false, false, false, false);
        assert_eq!(format!("{}", castle_move), "O-O-O");
    }

    #[test]
    fn test_promotion_move() {
        let promo_move = Move::new_all_fields(48, 56, Some(Piece::WhiteQueen), false, false, false, false, false, false, false, false);
        assert_eq!(format!("{}", promo_move), "A7A8=Q");
    }

    #[test]
    fn test_promotion_capture_move() {
        let promo_capture = Move::new_all_fields(53, 62, Some(Piece::BlackKnight), true, false, false, false, false, false, false, false);
        assert_eq!(format!("{}", promo_capture), "F7xG8=N");
    }

    #[test]
    fn test_move_check() {
        let check_move = Move::new_all_fields(51, 59, None, false, false, false, false, true, false, false, false);
        assert_eq!(format!("{}", check_move), "D7D8+");
    }

    #[test]
    fn test_move_checkmate() {
        let mate_move = Move::new_all_fields(60, 52, None, false, false, false, false, false, true, false, false);
        assert_eq!(format!("{}", mate_move), "E8E7#");
    }

    #[test]
    fn test_full_move_all_conditions() {
        let complex_move = Move::new_all_fields(52, 61, Some(Piece::WhiteRook), true, false, false, false, true, false, false, false);
        assert_eq!(format!("{}", complex_move), "E7xF8=R+");
    }
}