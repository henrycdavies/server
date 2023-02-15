#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod server;
mod http;
mod thread;
mod website_handler;


fn main() {
    let default_port = String::from("8080");
    let port = env::var("RUST_SERVER_PORT").unwrap_or(default_port);
    let server_address = format!("127.0.0.1:{}", port);
    let server = Server::new(server_address);
    server.run(WebsiteHandler::new());
}
