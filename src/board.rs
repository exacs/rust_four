const WIDTH: usize = 7;
const HEIGHT: i32 = 6;

use std::fmt;

pub struct Board {
    columns: [i32; WIDTH],
}

impl Board {
    pub fn new() -> Board {
        Board {
            columns: [0; WIDTH]
        }
    }

    pub fn play(&mut self, index: usize) {
        self.columns[index] = self.columns[index] + 1;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for h in 0..HEIGHT {
            for w in self.columns.iter() {
                if *w >= HEIGHT - h {
                    write!(f, "X ")?
                } else {
                    write!(f, "· ")?
                }
            }
            writeln!(f)?
        }

        for n in 0..WIDTH {
            write!(f, "{} ", n)?
        }

        write!(f, "")
    }
}
