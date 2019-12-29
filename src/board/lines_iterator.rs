use super::*;

pub struct LinesIterator {
    width: i32,
    height: i32,
    i: i32,
    j: i32,
    k: usize,
}

impl LinesIterator {
    pub fn new(width: i32, height: i32) -> LinesIterator {
        LinesIterator {
            width,
            height,
            i: -1,
            j: 0,
            k: 0,
        }
    }
}

impl Iterator for LinesIterator {
    type Item = ((i32, i32), Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let directions = [
            Direction::South,
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
        ];

        self.i += 1;

        if self.i >= self.width {
            self.j += 1;
            self.i = 0;
        }

        if self.j >= self.height {
            self.k += 1;
            self.j = 0;
            self.i = 0;
        }

        if self.k >= directions.len() {
            return None;
        }

        return Some(((self.i, self.j), directions[self.k]));
    }
}
