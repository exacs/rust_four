mod board;

fn main() {
    use board::Board;

    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    let mut my_board = Board::new();
    my_board.play(1);
    my_board.play(1);
    my_board.play(2);
    println!("{}", my_board);
}
