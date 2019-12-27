pub mod board;
pub mod player;

use board::Board;
use board::BoardError;
use board::Direction;
use board::Piece as Color;
use player::Player;
use std::fmt;

pub struct Game {
    height: i32,
    width: i32,
    board: Board,
    turn: Option<Color>,
    winner: Option<Color>,
    black_player: Box<Player>,
    white_player: Box<Player>,
}

type Line = Vec<Option<Color>>;

fn color_of_line(line: &Line) -> Option<Color> {
    let color = line[0]?;

    if line.iter().all(|&x| x == Some(color)) {
        return Some(color);
    } else {
        return None;
    }
}

impl Game {
    pub fn new(
        width: i32,
        height: i32,
        white_player: Box<Player>,
        black_player: Box<Player>,
    ) -> Game {
        Game {
            width,
            height,
            black_player,
            white_player,
            board: Board::new(width, height),
            turn: Some(Color::Black),
            winner: None,
        }
    }

    pub fn run(&mut self) {
        while self.turn != None {
            let next_movement = match self.turn.unwrap() {
                Color::Black => self.black_player.next_movement(&self),
                Color::White => self.white_player.next_movement(&self),
            };

            self.play(next_movement)
        }
    }

    fn guess_winner(&mut self) {
        let directions = [
            Direction::South,
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
        ];

        let mut combos: Vec<((i32, i32), Direction)> = Vec::new();

        for i in 0..self.width {
            for j in 0..self.height {
                for k in directions.iter() {
                    combos.push(((i, j), *k));
                }
            }
        }

        self.winner = combos
            .iter()
            .map(|&(point, dir)| self.board.get_line(&point, dir, 4))
            .find_map(|line| color_of_line(&line));
    }

    fn play(&mut self, index: i32) {
        if self.turn == None {
            return;
        }

        match self.board.play(index, self.turn.unwrap()) {
            Ok(()) => match self.turn.unwrap() {
                Color::Black => self.turn = Some(Color::White),
                Color::White => self.turn = Some(Color::Black),
            },
            Err(BoardError::FullColumn) => return,
            Err(_) => panic!("Error when playing"),
        }
        self.guess_winner();

        if self.winner != None {
            self.turn = None;
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.board)?;

        match self.winner {
            None => (),
            Some(Color::Black) => writeln!(
                f,
                "{} (X) won",
                self.black_player.name().unwrap_or("BLACK".to_string())
            )?,
            Some(Color::White) => writeln!(
                f,
                "{} (O) won",
                self.white_player.name().unwrap_or("WHITE".to_string())
            )?,
        }

        match self.turn {
            None => write!(f, "Game has finished")?,
            Some(Color::Black) => write!(
                f,
                "Its {} (X) turn",
                self.black_player.name().unwrap_or("BLACK".to_string())
            )?,
            Some(Color::White) => write!(
                f,
                "Its {} (O) turn",
                self.white_player.name().unwrap_or("WHITE".to_string())
            )?,
        }

        write!(f, "")
    }
}
