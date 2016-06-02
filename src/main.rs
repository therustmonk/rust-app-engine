#[macro_use]
extern crate rustful;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::error::Error;

use rustful::{Server, Context, Response, TreeRouter};

fn response_ok(_: Context, response: Response) {
    response.send("OK!");
}

fn main() {
    env_logger::init().unwrap();
    let server_result = Server {
        host: 8080.into(),
        handlers: insert_routes!{
            TreeRouter::new() => {
                Get: response_ok,
                "_ah" => {
                    Get: response_ok,
                    "start" => Get: response_ok,
                    "stop" => Get: response_ok,
                    "health" => Get: response_ok
                }
            }
        },
        ..Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(e) => error!("could not start server: {}", e.description()),
    }
}
