/**
 * Rust test application with the purpose of testing various 
 * features rust has to offer.
 *
 * @author: Colin Kirkpatrick
 * @version: 0.1
 */

extern crate rand;

mod guess; // Import for the local guessing game

/// Main function for test appication.
fn main() {
    guess::guessing_game();
}
