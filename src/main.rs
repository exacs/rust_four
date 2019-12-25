use std::fmt;

const WIDTH: usize = 7;

struct Board {
    columns: [i32; WIDTH],
}

impl Board {
    fn new() -> Board {
        Board {
            columns: [0; WIDTH]
        }
    }

    fn play(&mut self, index: usize) {
        self.columns[index] = self.columns[index] + 1;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let height: i32 = 6;

        for h in 0..height {
            for w in self.columns.iter() {
                if *w >= height - h {
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


        writeln!(f, "")
    }
}

fn main() {
    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    let mut my_board = Board::new();
    my_board.play(1);
    my_board.play(1);
    my_board.play(2);
    println!("{}", my_board);
}
