use crate::http::{Response, StatusCode, ParseError};
use std::net::TcpListener;
use std::io::{Read, Write};
use std::convert::TryFrom;
use std::thread::{self, Thread};
use crate::http::Request;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e:  &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        let pool = ThreadPool::new(4);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    pool.execute(|| {
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                println!("Received a request: {}", String::from_utf8_lossy(&buffer));
    
                                let response = match Request::try_from(&buffer[..]) {
                                    Ok(request) => {
                                        handler.handle_request(&request)
                                    },
                                    Err(e) => {
                                        handler.handle_bad_request(&e)
                                    },
                                };
    
                                if let Err(e) = response.send(&mut stream) {
                                    println!("Failed to send response: {}", e);
                                }
                            },
                            Err(e) => println!("Failed to read from connection: {}", e),
                        }
                    });
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}