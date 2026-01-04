use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust allows to shadow variables, so we can reuse the name `guess`
        // Expect will cause the program to crash if parse fails
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // Using match to handle the Result of parse
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Using cmp method, the code compares e.g. 50 to 38 and will return Ordering::Greater
        // The match expression gets the Ordering::Greater value and starts checking each arm’s pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}