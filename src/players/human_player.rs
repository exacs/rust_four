use crate::board::Color;
use crate::game::*;
use crate::players::Player;
use std::io;

#[allow(dead_code)]
pub struct HumanPlayer {
    name: String,
    color: Option<Color>,
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

    fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }
}
