mod game;
use game::player::HumanPlayer;
use game::player::RandomPlayer;
use game::Game;

fn main() {
    let p1 = HumanPlayer::new();
    let p2 = RandomPlayer::new();

    let mut my_game = Game::new(7, 6, Box::new(p1), Box::new(p2));

    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    my_game.run();
    println!("{}", my_game);
}
