use std::fmt;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Debug)]
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
    width: i32,
    height: i32,
}

pub enum Direction {
    South,
    East,
    SouthEast,
    SouthWest,
}

type Coords = (i32, i32);

impl Board {
    pub fn new(width: i32, height: i32) -> Board {
        if width <= 0 || height <= 0 {
            panic!("Board width and height should be positive");
        }

        Board {
            columns: (0..width).map(|i| (i, width*(height-1) + i)).collect(),
            positions: HashMap::new(),
            height,
            width,
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
        self.columns.insert(index, pos - self.width);

        Ok(())
    }

    pub fn get(&self, (row, cell): Coords) -> Option<&Piece> {
        if row < 0 || cell < 0 || row >= self.height || cell >= self.width {
            return None;
        }

        let pos = row * self.width + cell;
        self.positions.get(&pos)
    }

    pub fn get_line(&self, (x, y): Coords, d: Direction, length: usize) -> Vec<Option<&Piece>> {
        let inc: Coords = match d {
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
        };

        (0..length)
            .map(|i| i as i32)
            .map(|i| (x + inc.0 * i, y + inc.1 * i))
            .map(|i| self.get(i))
            .collect()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                match self.get((i, j)) {
                    None => write!(f, "· ")?,
                    Some(Piece::Black) => write!(f, "X ")?,
                    Some(Piece::White) => write!(f, "O ")?,
                }
            }
            writeln!(f)?
        }

        for n in 0..self.width {
            write!(f, "{} ", n)?
        }

        writeln!(f, "")
    }
}
