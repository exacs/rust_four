pub mod database;

use crate::game::board::Piece;
use crate::game::player::Player;
use crate::game::Game;

pub struct SmartPlayer {
    dumb_player: Box<Player>,
    color: Option<Piece>,
}

fn save_winner_sequence(seq: &[i32]) {
    database::save_seq("winner", seq);

    match seq.split_last() {
        None => (),
        Some((_, seq2)) => save_loser_sequence(seq2),
    }
}

fn save_loser_sequence(seq: &[i32]) {
    database::save_seq("loser", seq);
}

fn save_draw_sequence(seq: &[i32]) {
    database::save_seq("draw", seq);
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
            save_draw_sequence(game.get_board().get_sequence());
        } else if game.get_winner() == self.color {
            save_winner_sequence(game.get_board().get_sequence());
        } else {
            save_winner_sequence(game.get_board().get_sequence());
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
