/**
 * Rust test application with the purpose of testing various 
 * features rust has to offer.
 *
 * @author: Colin Kirkpatrick
 * @version: 0.1
 * @resource: rand
 */

extern crate hyper;
extern crate iron;
extern crate time;

use std::thread;

//mod guess;
mod server; // Import for the local server.rs
mod client; // Import for the client
 
/// Main function for test appication: Will take advantage of concurrency.
fn main() {
    server::server_start();
    //thread::spawn(|| server::server_start());
    //thread::spawn(|| client::client_start());

}

