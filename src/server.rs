use crate::http::{Response, Request, StatusCode, ParseError};
use crate::thread::ThreadPool;
use std::net::TcpListener;
use std::io::{Read};
use std::convert::TryFrom;
use std::thread;

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

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    // Handler must implement Send to be able to able to be 
    pub fn run(self, mut handler: impl Handler + Send + 'static + Copy) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        let pool = ThreadPool::new(4);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    pool.execute(move || {
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                println!("{}", String::from_utf8_lossy(&buffer));
    
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