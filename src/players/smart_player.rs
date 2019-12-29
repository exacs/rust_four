use crate::database;
use crate::game::board::Piece;
use crate::game::Game;
use crate::players::Player;
use rand::prelude::*;

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

fn get_loser_movements(seq: &[i32]) -> Vec<i32> {
    let mut banned = Vec::new();

    for s1 in database::read_seq("loser", seq.len() + 1) {
        let banned_sequence = s1.split_last().unwrap();

        if seq == banned_sequence.1 {
            banned.push(*banned_sequence.0);
        }
    }

    return banned;
}

#[allow(dead_code)]
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
        let seq = &game.get_board().get_sequence()[..];
        let banned = get_loser_movements(seq);

        let mut rng = thread_rng();
        let mut options = vec![];

        for column in game.get_board().get_unfilled_columns() {
            if !banned.contains(&column) {
                options.push(column);
            }
        }

        if options.len() > 0 {
            return *options.choose(&mut rng).unwrap();
        } else {
            return self.dumb_player.next_movement(game);
        }
    }

    fn set_color(&mut self, color: Piece) {
        self.color = Some(color);
    }
}
