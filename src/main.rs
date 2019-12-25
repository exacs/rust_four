const WIDTH: usize = 7;

fn main() {
    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    let mut board: [i32; WIDTH] = [0; WIDTH];

    print_board(&board);
    play_piece(&mut board, 1);
    play_piece(&mut board, 1);
    play_piece(&mut board, 2);
    print_board(&board);
}

fn print_board(board: &[i32; WIDTH]) {
    let height = 6;

    for h in 0..height {
        for w in board.iter() {
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

fn play_piece(board: &mut [i32; WIDTH], index: usize) {
    board[index] = board[index] + 1;
}
