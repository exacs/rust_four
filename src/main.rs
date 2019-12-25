mod game;
use game::Game;

fn main() {
    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    let mut my_game = Game::new();
    my_game.play(1);
    my_game.play(2);
    my_game.play(1);
    my_game.play(2);
    my_game.play(1);
    my_game.play(2);
    my_game.play(1);
    my_game.play(0);
    println!("{}", my_game);
}
