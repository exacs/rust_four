use crate::game::Game;
use std::io;
use rand::Rng;

pub struct HumanPlayer;
pub struct RandomPlayer;

impl HumanPlayer {
  pub fn next_movement(game: &Game) -> i32 {
    println!("{}", game);
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Failed to read line");

    return input.trim().parse::<i32>().expect("Not parseable!!");
  }
}

impl RandomPlayer {
  pub fn next_movement(game: &Game) -> i32 {
    let columns = game.board().get_unfilled_columns();
    let r = rand::thread_rng().gen_range(0, columns.len());

    return columns[r];
  }
}
