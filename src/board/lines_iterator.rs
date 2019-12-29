use super::*;

pub struct LinesIterator<'a> {
    board: &'a Board,
    directions: DirectionsIterator,
    length: i32,
}

impl<'a> LinesIterator<'a> {
    fn get_content(&self, ((x, y), d): ((i32, i32), Direction)) -> Vec<Option<&'a Color>> {
        let (xi, yi): Coords = match d {
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
        };

        let mut v = vec![];

        for i in 0..self.length {
            let coords = (x + xi * i, y + yi * i);
            let element = self.board.get(&coords);
            v.push(element);
        }

        return v;
    }

    pub fn new(board: &Board, directions: DirectionsIterator, length: i32) -> LinesIterator {
        LinesIterator {
            board,
            directions,
            length,
        }
    }
}

impl<'a> Iterator for LinesIterator<'a> {
    type Item = Vec<Option<&'a Color>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.directions.next() {
            None => None,
            Some(v) => Some(self.get_content(v)),
        }
    }
}
