use rand::Rng;
use std::cmp::Ordering;

use std::io;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100);
    println!("Your secret number is {}", secret);

    loop {
        println!("Please enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess.trim());

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please enter a positive number!");

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
