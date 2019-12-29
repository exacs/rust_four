mod human_player;
mod random_player;
mod smart_player;

use crate::board::*;
use crate::game::*;

pub type RandomPlayer = random_player::RandomPlayer;
pub type SmartPlayer = smart_player::SmartPlayer;

#[allow(dead_code)]
pub type HumanPlayer = human_player::HumanPlayer;

pub trait Player {
    fn name(&self) -> Option<String> {
        None
    }

    fn set_color(&mut self, _color: Color) {}

    fn next_movement(&self, game: &Game) -> i32;
}
