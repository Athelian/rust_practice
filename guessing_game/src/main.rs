use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=500);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("You guessed: {guess}.");

    let guessed: i32 = guess
        .trim()
        .parse()
        .expect("Failed to parse integer from string");

    match guessed.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
    }
}
