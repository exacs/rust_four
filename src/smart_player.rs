use crate::game::board::Piece;
use crate::game::player::Player;
use crate::game::Game;

pub struct SmartPlayer {
    dumb_player: Box<Player>,
    color: Option<Piece>,
}

impl SmartPlayer {
    pub fn new(dumb_player: Box<Player>) -> SmartPlayer {
        SmartPlayer {
            dumb_player,
            color: None,
        }
    }

    pub fn learn(&self, game: &Game) {
        if game.get_winner() == None {
            println!("This was a draw!");
        } else if game.get_winner() == self.color {
            println!("I've won in {} moves", game.get_board().get_sequence().len());
        } else {
            println!("I've lost! in {} moves", game.get_board().get_sequence().len());
        }
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

    fn set_color(&mut self, color: Piece) {
        self.color = Some(color);
    }
}
