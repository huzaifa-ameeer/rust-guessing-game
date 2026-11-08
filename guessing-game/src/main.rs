use std::cmp::Ordering;
use std::io;
use rand::prelude::*;
use colored::*;

fn main () {
    println!("------------------------");
    println!("Welcome to guessing game");

    let secret_number = rand::rng().random_range(1..5);
    let mut guess = String::new();

 loop {
    println!("Please input your guess");
    guess.clear();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

            println!("You guessed: {}", guess);

            let guess: u32 = guess.trim().parse().expect("Please type a number");

            match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
        }
        }
        
}