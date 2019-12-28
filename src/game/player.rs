use crate::game::board::Piece;
use crate::game::Game;
use rand::prelude::*;
use std::io;

pub trait Player {
    fn name(&self) -> Option<String> {
        None
    }

    fn set_color(&mut self, _color: Piece) {}

    fn next_movement(&self, game: &Game) -> i32;
}

#[allow(dead_code)]
pub struct HumanPlayer {
    name: String,
    color: Option<Piece>,
}

pub struct RandomPlayer {
    rng: ThreadRng,
}

#[allow(dead_code)]
impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        let mut input = String::new();
        println!("What is your name? ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        HumanPlayer {
            name: String::from(input.trim()),
            color: None,
        }
    }
}

impl Player for HumanPlayer {
    fn name(&self) -> Option<String> {
        return Some(self.name.to_string());
    }

    fn next_movement(&self, game: &Game) -> i32 {
        println!("{}", game);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        return input.trim().parse::<i32>().expect("Not parseable!!");
    }

    fn set_color(&mut self, color: Piece) {
        self.color = Some(color);
    }
}

impl RandomPlayer {
    pub fn new() -> RandomPlayer {
        RandomPlayer {
            rng: thread_rng(),
        }
    }
}

impl Player for RandomPlayer {
    fn name(&self) -> Option<String> {
        Some("Random IA".to_string())
    }

    fn next_movement(&self, game: &Game) -> i32 {
        let columns = game.board.get_unfilled_columns();
        let mut rng = self.rng;

        return *columns.choose(&mut rng).unwrap();
    }
}
