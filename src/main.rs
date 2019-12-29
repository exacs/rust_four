mod database;
mod game;
mod players;

use game::board::Piece;
use game::Game;
use players::*;

fn main() {
    let mut result = (0, 0);

    println!("Welcome to four-in-a-row!!! (implemented in Rust)");

    let mut p1 = SmartPlayer::new(Box::new(RandomPlayer::new()));
    let p2 = RandomPlayer::new();

    p1.set_color(Piece::Black);

    for _i in 0..1000 {
        let mut my_game = Game::new(7, 6, &p1, &p2);
        match my_game.run() {
            Some(Piece::Black) => result.0 = result.0 + 1,
            Some(Piece::White) => result.1 = result.1 + 1,
            _ => (),
        }
        p1.learn(&my_game);
    }
    println!("Result {:?}", result)
}
