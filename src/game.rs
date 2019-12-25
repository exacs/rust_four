mod board;

use std::fmt;
use board::Board;
use board::Piece as Player;

pub struct Game {
    board: Board,
    turn: Player,
}

impl Game {
    pub fn new() -> Game {
        let board = Board::new();

        Game {
            board: board,
            turn: Player::Black,
        }
    }

    pub fn play(&mut self, index: i32) {
        self.board.play(index, self.turn);
        match self.turn {
            Player::Black => self.turn = Player::White,
            Player::White => self.turn = Player::Black,
        }
    }
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      writeln!(f, "{}", self.board)?;

      match self.turn {
        Player::Black => write!(f, "Its BLACK turn"),
        Player::White => write!(f, "Its WHITE turn"),
      }
  }
}
