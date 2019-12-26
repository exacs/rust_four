use std::fmt;
use std::collections::HashMap;

type Coords = (i32, i32);

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
    positions: HashMap<Coords, Piece>,
    columns: HashMap<i32, i32>,
    width: i32,
    height: i32,
}

#[derive(Clone, Copy)]
pub enum Direction {
    South,
    East,
    SouthEast,
    SouthWest,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Board {
        if width <= 0 || height <= 0 {
            panic!("Board width and height should be positive");
        }

        Board {
            columns: (0..width).map(|i| (i, height-1)).collect(),
            positions: HashMap::new(),
            height,
            width,
        }
    }

    pub fn get_row(&self, column: i32) -> Result<i32, BoardError> {
        let pos = self.columns.get(&column)
            .ok_or(BoardError::NonValidColumn)?;

        if *pos < 0 {
            return Err(BoardError::FullColumn);
        }

        Ok(*pos)
    }

    pub fn get_unfilled_columns(&self) -> Vec<i32> {
        return self.columns.iter()
            .filter(|(&i, _)| self.get_row(i).is_ok())
            .map(|(&i, _)| i)
            .collect();
    }

    pub fn play(&mut self, index: i32, piece: Piece) -> Result<(), BoardError> {
        let row = self.get_row(index)?;

        self.positions.insert((row, index), piece);
        self.columns.insert(index, row - 1);

        Ok(())
    }

    pub fn get(&self, coords: &Coords) -> Option<Piece> {
        let v = self.positions.get(coords)?;

        Some(*v)
    }

    pub fn get_line(&self, (x, y): &Coords, d: Direction, length: usize) -> Vec<Option<Piece>> {
        let inc: Coords = match d {
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
        };

        (0..length)
            .map(|i| i as i32)
            .map(|i| (*x + inc.0 * i, *y + inc.1 * i))
            .map(|i| self.get(&i))
            .collect()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                match self.get(&(i, j)) {
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
