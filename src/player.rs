use crate::game::Game;
use std::io;

pub struct HumanPlayer;

impl HumanPlayer {
  pub fn next_movement(game: &Game) -> i32 {
    println!("{}", game);
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Failed to read line");

    return input.trim().parse::<i32>().expect("Not parseable!!");
  }
}
