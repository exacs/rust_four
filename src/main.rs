mod game;
mod player;
use game::Game;
use player::HumanPlayer;

fn main() {
    let mut my_game = Game::new(7, 6);
    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    while my_game.turn() != None {
        let a = HumanPlayer::next_movement(&my_game);
        my_game.play(a);
    }
    println!("{}", my_game);
}
