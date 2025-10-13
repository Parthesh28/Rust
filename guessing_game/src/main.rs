use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Let us play number guessing game");

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Enter your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read input");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                 println!("You win!");
                 break;
                },
        }
    }
}
