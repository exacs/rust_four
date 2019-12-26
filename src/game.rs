pub mod board;

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

    pub fn winner(&self) -> Option<Player> {
        return self.winner
    }

    pub fn turn(&self) -> Option<Player> {
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

        match self.winner() {
            None => (),
            Some(Player::Black) => writeln!(f, "BLACK won")?,
            Some(Player::White) => writeln!(f, "WHITE won")?,
        }

        match self.turn() {
            None => write!(f, "Game has finished")?,
            Some(Player::Black) => write!(f, "Its BLACK turn")?,
            Some(Player::White) => write!(f, "Its WHITE turn")?,
        }

        write!(f, "")
    }
}
