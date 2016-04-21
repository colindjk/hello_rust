/**
 * This file will include server setup, client side information, and basic HTML
 * manipulation using hyper and maybe some other stuff.
 *
 * @author: Colin Kirkpatrick
 * @version: 0.1
 * @resources: hyper
 */

extern crate hyper;
extern crate time;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;

/// Function for setting up the server.
pub fn server_start() {
    Server::http("127.0.0.1:3000")
        .unwrap()
        .handle(hello)
        .unwrap();
}

/// Hello world server caller.
fn hello(_: Request, res: Response) {
    res.send(b"Hello kibbles").unwrap();
}

