use crate::database;
use crate::game::board::Piece;
use crate::game::Game;
use crate::players::Player;
use rand::prelude::*;
use std::collections::HashMap;

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

fn count_conditions(filename: &str, seq: &[i32]) -> HashMap<i32, i32> {
    let mut count2: HashMap<i32, i32> = HashMap::new();

    for n in seq.len()..=42 {
        if n % 2 == seq.len() % 2 {
            continue;
        }

        for s1 in database::read_seq(filename, n) {
            let seq2 = s1.split_at(seq.len());

            if seq == seq2.0 {
                let first = s1[0];

                match count2.get(&first) {
                    None => count2.insert(first, 1),
                    Some(&v) => count2.insert(first, v + 1),
                };
            }
        }
    }

    return count2;
}

fn weighed_random(
    options: &Vec<i32>,
    positive_weights: HashMap<i32, i32>,
    negative_weights: HashMap<i32, i32>,
) -> &i32 {
    let mut weights = HashMap::new();
    let mut rng = thread_rng();

    let min_weight = negative_weights.values().max().unwrap_or(&0);

    for key in options {
        let neg = negative_weights.get(&key).unwrap_or(&0);
        let pos = negative_weights.get(&key).unwrap_or(&0);
        let value = 1 + *min_weight - neg + pos;
        weights.insert(key, value);
    }

    return options
        .choose_weighted(&mut rng, |item| weights.get(item).unwrap())
        .unwrap();
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
        let mut options = vec![];

        for column in game.get_board().get_unfilled_columns() {
            if !banned.contains(&column) {
                options.push(column);
            }
        }

        if options.len() > 0 {
            return *weighed_random(
                &options,
                count_conditions("winner", seq),
                count_conditions("loser", seq),
            );
        } else {
            return self.dumb_player.next_movement(game);
        }
    }

    fn set_color(&mut self, color: Piece) {
        self.color = Some(color);
    }
}
