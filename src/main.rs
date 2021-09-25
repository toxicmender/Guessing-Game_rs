use std::io;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret = rand::thread_rng().gen_range(1..101);

    println!("The Secret number: {}", secret);

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
