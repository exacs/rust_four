fn main() {
    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    print_board();
}

fn print_board() {
    let width = 7;
    let height = 6;

    for _h in 0..height {
        for _w in 1..width+1 {
            print!("· ")
        }
        println!()
    }
    for n in 1..width+1 {
        print!("{} ", n)
    }
    println!()
}
