use crate::game::Game;
use std::io;

pub trait Player {
    fn name(&self) -> Option<String> {
        None
    }

    fn next_movement(&self, game: &Game) -> i32;
}

pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        let mut input = String::new();
        println!("What is your name? ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        HumanPlayer {
            name: String::from(input.trim()),
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
}

/*
impl RandomPlayer {
  pub fn next_movement(game: &Game) -> i32 {
    let columns = game.board().get_unfilled_columns();
    let r = rand::thread_rng().gen_range(0, columns.len());

    return columns[r];
  }
}
*/
