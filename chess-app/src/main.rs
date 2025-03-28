use chess_core::game::bitboard::Bitboard;

fn main() {
    let empty_board = Bitboard::new();
    println!("Empty Bitboard: {}", empty_board);

    let full_board = Bitboard::from(u64::MAX);
    println!("Full Bitboard: {}", full_board);

    let pattern_board = Bitboard::from(0b10001000);
    println!("Pattern Bitboard: {}", pattern_board);
}