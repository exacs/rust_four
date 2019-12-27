pub mod board;
pub mod player;

use board::Board;
use board::BoardError;
use board::Piece as Color;
use player::Player;
use std::fmt;

pub struct Game<'a> {
    board: Board,
    turn: Option<Color>,
    winner: Option<Color>,
    black_player: &'a Player,
    white_player: &'a Player,
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

impl<'a> Game<'a> {
    pub fn new(
        width: i32,
        height: i32,
        black_player: &'a Player,
        white_player: &'a Player,
    ) -> Game<'a> {
        Game {
            black_player,
            white_player,
            board: Board::new(width, height),
            turn: Some(Color::Black),
            winner: None,
        }
    }

    pub fn run(&mut self) -> Option<Color> {
        while self.turn != None {
            let next_movement = match self.turn.unwrap() {
                Color::Black => self.black_player.next_movement(&self),
                Color::White => self.white_player.next_movement(&self),
            };

            self.play(next_movement)
        }

        return self.winner;
    }

    fn guess_winner(&mut self) {
        self.winner = self.board.all_iter()
            .map(|(point, dir)| self.board.get_line(&point, dir, 4))
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

impl<'a> fmt::Display for Game<'a> {
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
