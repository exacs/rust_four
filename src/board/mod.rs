mod fmt;
mod lines_iterator;

use lines_iterator::LinesIterator;
use std::collections::HashMap;

type Coords = (i32, i32);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, PartialEq)]
pub enum BoardError {
    NonValidColumn,
    FullColumn,
}

pub struct Board {
    positions: HashMap<Coords, Color>,
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
            columns: (0..width).map(|i| (i, height - 1)).collect(),
            positions: HashMap::new(),
            height,
            width,
        }
    }

    fn get_row(&self, column: i32) -> Result<i32, BoardError> {
        let pos = self
            .columns
            .get(&column)
            .ok_or(BoardError::NonValidColumn)?;

        if *pos < 0 {
            return Err(BoardError::FullColumn);
        }

        Ok(*pos)
    }

    pub fn get_unfilled_columns(&self) -> Vec<i32> {
        return self
            .columns
            .iter()
            .filter(|(&i, _)| self.get_row(i).is_ok())
            .map(|(&i, _)| i)
            .collect();
    }

    pub fn play(&mut self, index: i32, piece: Color) -> Result<(), BoardError> {
        let row = self.get_row(index)?;

        self.positions.insert((row, index), piece);
        self.columns.insert(index, row - 1);

        Ok(())
    }

    pub fn get(&self, coords: &Coords) -> Option<&Color> {
        self.positions.get(coords)
    }

    pub fn get_line(&self, (x, y): &Coords, d: Direction, length: usize) -> Vec<Option<&Color>> {
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

    pub fn all_iter(&self) -> LinesIterator {
        LinesIterator::new(self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn create_invalid_size() {
        Board::new(0, 0);
    }

    #[test]
    fn create_valid_size() {
        Board::new(1, 1);
    }

    #[test]
    fn get_row_coords() {
        let b = Board::new(1, 1);
        assert_eq!(b.get_row(0).unwrap(), 0);
    }

    #[test]
    fn get_full_column() -> Result<(), BoardError> {
        let mut b = Board::new(1, 1);
        b.play(0, Color::White)?;

        if b.get_row(0).unwrap_err() == BoardError::FullColumn {
            Ok(())
        } else {
            panic!();
        }
    }
}
