/**
 * This file will include client side setup for comminication with the
 * server file.
 *
 * @author: Colin Kirkpatrick
 * @version: 0.1
 * @resources: hyper
 */

extern crate hyper;
extern crate time;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

pub fn client_start() {
    // User new() to create client
    let client = Client::new();

    // Create outgoing request, for this situation will use loopback.
    let mut res = client.get("localhost:3000")
        .header( Connection::close() ) // Create header
        .send().unwrap();

    // Read response
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}

