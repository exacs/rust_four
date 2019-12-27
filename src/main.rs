mod game;
mod smart_player;

// use game::player::HumanPlayer;
use game::player::RandomPlayer;
use smart_player::SmartPlayer;
use game::Game;

fn main() {
    let p1 = SmartPlayer::new(Box::new(RandomPlayer::new()));
    let p2 = RandomPlayer::new();

    let mut my_game = Game::new(7, 6, Box::new(p1), Box::new(p2));

    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    println!("Winner is {:?}", my_game.run());
}
