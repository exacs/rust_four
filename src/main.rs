mod game;
mod player;
use game::Game;

fn main() {
    let mut my_game = Game::new(7, 6);
    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    while my_game.turn() != None {
        let a = player::human_player(&my_game);
        my_game.play(a);
    }
    println!("{}", my_game);
}
