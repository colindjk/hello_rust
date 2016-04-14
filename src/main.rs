extern crate rand;

use std::io;
use std::comp::Ordering;
use rand::Rng;

/// Random number guessing game.
fn main()
{
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret_number is {}", secret_number);
    println!("Please input your guess:");

    let mut guess = String::new();



}
