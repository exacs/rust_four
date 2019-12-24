use std::io;

fn main() {
    println!("Welcome to four-in-a-row!!! (implemented in Rust)");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
