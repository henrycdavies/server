use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;
use std::time::Duration;
use std::{env, fs, thread};

// TODO: Rearchitect so that we can have heap-allocated properties on this struct
#[derive(Clone, Copy)]
pub struct WebsiteHandler {
}

impl WebsiteHandler {
    pub fn new() -> Self{
        Self { }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("RUST_SERVER_PUBLIC_PATH").unwrap_or(default_path);
        let path = format!("{}/{}", public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/sleep" => {
                    thread::sleep(Duration::from_secs(10));
                    Response::new(StatusCode::Ok, Some("OK".to_string()))
                },
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}