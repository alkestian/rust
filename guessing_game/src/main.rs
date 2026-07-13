use rand::Rng;
use std::cmp::Ordering;

use std::io;

fn main() {
    println!("Guess the number between 1 and 10!");

    let secret = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Match!");
                break;
            }
        }
    }
}
