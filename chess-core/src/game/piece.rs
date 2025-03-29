use std::fmt::Display;

pub enum Piece {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_str = match self {
            Piece::WhitePawn | Piece::BlackPawn => "",
            Piece::WhiteRook | Piece::BlackRook => "R",
            Piece::WhiteKnight | Piece::BlackKnight => "N",
            Piece::WhiteBishop | Piece::BlackBishop => "B",
            Piece::WhiteQueen | Piece::BlackQueen => "Q",
            Piece::WhiteKing | Piece::BlackKing => "K",
        };
        write!(f, "{}", piece_str)
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_piece_display() {
        let pieces = vec![
            (Piece::WhitePawn, ""),
            (Piece::WhiteRook, "R"),
            (Piece::WhiteKnight, "N"),
            (Piece::WhiteBishop, "B"),
            (Piece::WhiteQueen, "Q"),
            (Piece::WhiteKing, "K"),
            (Piece::BlackPawn, ""),
            (Piece::BlackRook, "R"),
            (Piece::BlackKnight, "N"),
            (Piece::BlackBishop, "B"),
            (Piece::BlackQueen, "Q"),
            (Piece::BlackKing, "K"),
        ];

        for (piece, expected_str) in pieces {
            assert_eq!(piece.to_string(), expected_str, "Failed for piece: {}", piece);
        }
    }
}