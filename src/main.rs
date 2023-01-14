fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }
    
    impl Server {
        pub fn new(addr: String) -> Self {
            Server {
                addr
            }
        }
    
        pub fn run(self) {
            println!("Listening on {}", self.addr);
        }
    }
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {

}
/*
GET /user/?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
