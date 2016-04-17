/**
 * Guessing game which has the user guess a value between 1
 * and 100, giving hints (too large / small) per guess
 *
 * @author: Colin Kirkpatrick
 * @version: 0.1
 */
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

/// Random number guessing game.
pub fn guessing_game() {
    println!("Guess the number");
    // Below defaults to i32 (signed 32-bit integer)
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // so, parse() returns an enum, with the 'Ok' -thing- or the 'Err' type.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // if Okay, write to 'num', return num.
            Err(_) => continue, // continue: next iter of 'loop'
        };

        println!("You guessed {}", guess);

        // The 'Ordering' value is an 'enum'; which looks like this: enum foo { Bar, Biz }; 
        // where Bar and Biz are different types of Foo
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
