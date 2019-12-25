mod board;

fn main() {
    use board::Board;
    use board::Piece;

    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    let mut my_board = Board::new();
    my_board.play(1, Piece::Black);
    my_board.play(1, Piece::White);
    my_board.play(1, Piece::White);
    my_board.play(1, Piece::White);
    my_board.play(1, Piece::White);
    my_board.play(1, Piece::White);
    my_board.play(1, Piece::White);
    my_board.play(2, Piece::White);
    println!("{}", my_board);
}
