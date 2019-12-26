pub mod board;

use std::fmt;
use board::Board;
use board::BoardError;
use board::Piece as Color;
use board::Direction;

pub struct Game {
    height: i32,
    width: i32,
    board: Board,
    turn: Color,
    winner: Option<Color>,
}

type Line = Vec<Option<Color>>;

fn color_of_line (line: &Line) -> Option<Color> {
    let color = line[0]?;

    if line.iter().all(|&x| x == Some(color)) {
        return Some(color);
    } else {
        return None;
    }
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width,
            height,
            board: Board::new(width, height),
            turn: Color::Black,
            winner: None,
        }
    }

    fn guess_winner(&mut self) {
        let directions = [
            Direction::South,
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
        ];

        let mut combos:Vec<((i32, i32), Direction)> = Vec::new();

        for i in 0..self.width {
            for j in 0..self.height {
                for k in directions.iter() {
                    combos.push(((i, j), *k));
                }
            }
        }

        self.winner = combos.iter()
            .map(|&(point, dir)| self.board.get_line(&point, dir, 4))
            .find_map(|line| color_of_line(&line));
    }

    pub fn board(&self) -> &Board {
        return &self.board;
    }

    pub fn winner(&self) -> Option<Color> {
        return self.winner
    }

    pub fn turn(&self) -> Option<Color> {
        match self.winner {
            None => Some(self.turn),
            _ => None,
        }
    }

    pub fn play(&mut self, index: i32) {
        if self.turn() == None {
            return;
        }

        match self.board.play(index, self.turn) {
            Ok(()) => match self.turn {
                Color::Black => self.turn = Color::White,
                Color::White => self.turn = Color::Black,
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

        match self.winner() {
            None => (),
            Some(Color::Black) => writeln!(f, "BLACK won")?,
            Some(Color::White) => writeln!(f, "WHITE won")?,
        }

        match self.turn() {
            None => write!(f, "Game has finished")?,
            Some(Color::Black) => write!(f, "Its BLACK turn")?,
            Some(Color::White) => write!(f, "Its WHITE turn")?,
        }

        write!(f, "")
    }
}
