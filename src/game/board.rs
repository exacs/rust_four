const WIDTH: i32 = 7;
const HEIGHT: i32 = 6;

use std::fmt;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Piece {
    Black,
    White,
}

pub enum BoardError {
    NonValidColumn,
    FullColumn,
}

pub struct Board {
    positions: HashMap<i32, Piece>,
    columns: HashMap<i32, i32>,
}

type Coords = (i32, i32);

impl Board {
    pub fn new() -> Board {
        let mut columns = HashMap::new();

        for i in 0..WIDTH {
            columns.insert(i, WIDTH*(HEIGHT-1) + i);
        }

        Board {
            columns: columns,
            positions: HashMap::new(),
        }
    }

    pub fn play(&mut self, index: i32, piece: Piece) -> Result<(), BoardError> {
        let pos = self.columns.get(&index)
            .ok_or(BoardError::NonValidColumn)?;

        let pos = *pos;

        if pos < 0 {
            return Err(BoardError::FullColumn);
        }

        self.positions.insert(pos, piece);
        self.columns.insert(index, pos - WIDTH);

        Ok(())
    }

    pub fn get(&self, (row, cell): Coords) -> Option<&Piece> {
        if row < 0 || cell < 0 || row >= HEIGHT || cell >= WIDTH {
            return None;
        }

        let pos = row * WIDTH + cell;
        self.positions.get(&pos)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                match self.get((i, j)) {
                    None => write!(f, "· ")?,
                    Some(Piece::Black) => write!(f, "X ")?,
                    Some(Piece::White) => write!(f, "O ")?,
                }
            }
            writeln!(f)?
        }

        for n in 0..WIDTH {
            write!(f, "{} ", n)?
        }

        writeln!(f, "")
    }
}
