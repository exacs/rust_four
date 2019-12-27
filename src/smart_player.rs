use crate::game::player::Player;
use crate::game::Game;

pub struct SmartPlayer {
    dumb_player: Box<Player>,
}

impl SmartPlayer {
    pub fn new(dumb_player: Box<Player>) -> SmartPlayer {
        SmartPlayer { dumb_player }
    }
}

impl Player for SmartPlayer {
    fn name(&self) -> Option<String> {
        Some("Smart".to_string())
    }

    fn next_movement(&self, game: &Game) -> i32 {
        // println!("Hello, I am smart!");
        self.dumb_player.next_movement(game)
    }
}
