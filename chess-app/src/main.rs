use chess_core::pieces::piece::Piece;

fn main() {
    let pieces = vec![
        Piece::WhitePawn,
        Piece::WhiteRook,
        Piece::WhiteKnight,
        Piece::WhiteBishop,
        Piece::WhiteQueen,
        Piece::WhiteKing,
        Piece::BlackPawn,
        Piece::BlackRook,
        Piece::BlackKnight,
        Piece::BlackBishop,
        Piece::BlackQueen,
        Piece::BlackKing,
    ];

    for piece in &pieces {
        print!("{} ", piece);
    }
    println!();
}