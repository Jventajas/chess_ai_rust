use chess_core::game::state::GameState;

fn main() {
    let game_state = GameState::new_game();
    println!("{}", game_state);
}