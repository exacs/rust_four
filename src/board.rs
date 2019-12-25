const WIDTH: i32 = 7;
const HEIGHT: i32 = 6;

use std::fmt;
use std::collections::HashMap;

pub enum Piece {
    Black,
    White,
}

pub struct Board {
    positions: HashMap<i32, Piece>,
    columns: HashMap<i32, i32>,
}

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

    pub fn play(&mut self, index: i32, piece: Piece) {
        let pos = *self.columns.get(&index).expect("No position found");

        self.positions.insert(pos, piece);
        self.columns.insert(index, pos - WIDTH);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let pos = i * WIDTH + j;

                match self.positions.get(&pos) {
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
