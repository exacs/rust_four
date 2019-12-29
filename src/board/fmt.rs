use super::*;
use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                match self.get(&(i, j)) {
                    None => write!(f, "· ")?,
                    Some(Color::Black) => write!(f, "X ")?,
                    Some(Color::White) => write!(f, "O ")?,
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
