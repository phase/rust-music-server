extern crate hyper;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;

fn hello(_: Request, res: Response) {
    res.send(b"Hey").unwrap();
}

fn main() {
    Server::http("127.0.0.1:666").unwrap().handle(hello).unwrap();
}
