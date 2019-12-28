use crate::game::board::Piece;
use crate::game::Game;
use crate::players::Player;
use crate::database::Database;
use rand::prelude::*;

pub struct SmartPlayer {
    dumb_player: Box<Player>,
    color: Option<Piece>,
}

fn save_winner_sequence(seq: &[i32]) {
    Database::save_seq("winner", seq);

    match seq.split_last() {
        None => (),
        Some((_, seq2)) => save_loser_sequence(seq2),
    }
}

fn save_loser_sequence(seq: &[i32]) {
    Database::save_seq("loser", seq);
}

fn save_draw_sequence(seq: &[i32]) {
    Database::save_seq("draw", seq);
}

fn get_loser_sequence(seq: &[i32]) -> Vec<i32> {
    let mut banned = Vec::new();

    for s1 in Database::read_seq("loser", seq.len() + 1) {
        let banned_sequence = s1.split_last().unwrap();

        if seq == banned_sequence.1 {
            banned.push(*banned_sequence.0);
        }
    }

    return banned;
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
        let options = game.get_board().get_unfilled_columns();
        let mut rng = thread_rng();

        let seq = &game.get_board().get_sequence()[..];
        let banned = get_loser_sequence(seq);

        if banned.len() > 0 {
            println!("Found {}", banned.len());
        }

        self.dumb_player.next_movement(game);
        return *options.choose(&mut rng).unwrap();
    }

    fn set_color(&mut self, color: Piece) {
        self.color = Some(color);
    }
}
