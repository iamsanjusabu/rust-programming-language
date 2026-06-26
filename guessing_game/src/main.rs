use std::io::{self, Write};
use rand::RngExt;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game!");

    let secret_number: i32 = rand::rng().random_range(1..=100);

    loop {
        print!("Guess a number from 1 to 100: ");
        io::stdout().flush().unwrap();

        let mut guess: String = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit" {
            println!("Exiting the program!");
            break;
        }

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Numbers only");
                continue;
            },
        };

        match guess.cmp(& secret_number) {
            Ordering::Greater => println!("Guessed high"),
            Ordering::Less => println!("Guessed low"),
            Ordering::Equal => {
                println!("You won");
                break;
            },
        }
    }
}

