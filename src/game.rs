mod board;

use std::fmt;
use board::Board;
use board::Piece;

pub struct Game {
    board: Board,
    turn: Piece,
}

impl Game {
    pub fn new() -> Game {
        let board = Board::new();

        Game {
            board: board,
            turn: Piece::Black,
        }
    }

    pub fn play(&mut self, index: i32) {
        self.board.play(index, self.turn);
        match self.turn {
            Piece::Black => self.turn = Piece::White,
            Piece::White => self.turn = Piece::Black,
        }
    }
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      writeln!(f, "{}", self.board)?;

      match self.turn {
        Piece::Black => write!(f, "Its BLACK turn"),
        Piece::White => write!(f, "Its BLACK turn"),
      }
  }
}
