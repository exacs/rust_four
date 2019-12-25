mod board;

use std::fmt;
use board::Board;
use board::BoardError;
use board::Piece as Player;
use board::Direction;

pub struct Game {
    height: i32,
    width: i32,
    board: Board,
    turn: Player,
    winner: Option<Player>,
}

type Line = Vec<Option<Player>>;

fn color_of_line (line: &Line) -> Option<Player> {
    let color = line[0]?;

    if line.iter().all(|&x| x == Some(color)) {
        return Some(color);
    } else {
        return None;
    }
}

fn color_of_lines (lines: [Line; 4]) -> Option<Player> {
    lines.iter()
        .map(|line| color_of_line(line))
        .find(|&color| color != None)?
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width,
            height,
            board: Board::new(width, height),
            turn: Player::Black,
            winner: None,
        }
    }

    fn guess_winner(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let line1 = self.board.get_line((i, j), Direction::South, 4);
                let line2 = self.board.get_line((i, j), Direction::East, 4);
                let line3 = self.board.get_line((i, j), Direction::SouthEast, 4);
                let line4 = self.board.get_line((i, j), Direction::SouthWest, 4);

                match color_of_lines([line1, line2, line3, line4]) {
                    None => (),
                    Some(c) => {
                        self.winner = Some(c);
                        return
                    },
                }
            }
        }
    }

    pub fn play(&mut self, index: i32) {
        match self.board.play(index, self.turn) {
            Ok(()) => match self.turn {
                Player::Black => self.turn = Player::White,
                Player::White => self.turn = Player::Black,
            }
            Err(BoardError::FullColumn) => return,
            Err(_) => panic!("Error when playing"),
        }
        self.guess_winner()
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.board)?;

        match self.winner {
            None => (),
            Some(Player::Black) => writeln!(f, "BLACK won")?,
            Some(Player::White) => writeln!(f, "WHITE won")?,
        }

        match self.turn {
            Player::Black => write!(f, "Its BLACK turn"),
            Player::White => write!(f, "Its WHITE turn"),
        }
    }
}
