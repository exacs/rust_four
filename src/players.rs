pub mod human_player;
pub mod random_player;

use crate::game::board::*;
use crate::game::*;

pub type RandomPlayer = random_player::RandomPlayer;

#[allow(dead_code)]
pub type HumanPlayer = human_player::HumanPlayer;

pub trait Player {
    fn name(&self) -> Option<String> {
        None
    }

    fn set_color(&mut self, _color: Piece) {}

    fn next_movement(&self, game: &Game) -> i32;
}
