const WIDTH: i32 = 7;
const HEIGHT: i32 = 6;

use std::fmt;
use std::collections::HashMap;

pub struct Board {
    positions: HashMap<i32, bool>,
    columns: HashMap<i32, i32>,
}

impl Board {
    pub fn new() -> Board {
        let mut positions = HashMap::new();
        let mut columns = HashMap::new();

        for i in 0..WIDTH*HEIGHT {
            positions.insert(i, false);
        }

        for i in 0..WIDTH {
            columns.insert(i, WIDTH*(HEIGHT-1) + i);
        }

        Board {
            columns: columns,
            positions: positions,
        }
    }

    pub fn play(&mut self, index: i32) {
        let pos = *self.columns.get(&index).expect("No position found");

        self.positions.insert(pos, true);
        self.columns.insert(index, pos - WIDTH);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let pos = i * WIDTH + j;
                let val = self.positions.get(&pos)
                    .expect(&format!("There is nothing in position {}", pos));

                match val {
                    true => write!(f, "X ")?,
                    false => write!(f, "· ")?,
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
