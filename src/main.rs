use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    let secret: u32 = rand::thread_rng().gen_range(1..101);

    println!("The Secret number: {}", secret);

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret) {
        Ordering::Less => println!("too small!"),
        Ordering::Greater => println!("too big!"),
        Ordering::Equal => println!("You Win!"),
    }
    println!("You guessed: {}", guess);
}
