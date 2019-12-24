
fn main() {
    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    let width = 7;
    let mut board: Vec<i32> = Vec::new();

    for _v in 0..width {
        board.push(0);
    }

    print_board(&board);
}

fn print_board(board: &Vec<i32>) {
    let height = 6;

    for h in 0..height {
        for w in board {
            if *w >= height - h {
                print!("X ");
            } else {
                print!("· ");
            }
        }
        println!();
    }

    for n in 0..board.len() {
        print!("{} ", n);
    }

    println!()
}
