use std::net::TcpListener;
use crate::http::handle_client;

#[derive(Debug)]
pub struct Server {
    addr: String,
    listener: TcpListener,
}

impl Server {
    pub fn new(addr: Option<String>) -> Self {
        let addr = match addr {
            Some(addr) => addr,
            None => "localhost:2468".to_string(),
        };
        Self {
            addr: addr.to_string(),
            listener: match TcpListener::bind(addr) {
                Ok(listener) => listener,
                Err(e) => {
                    panic!("Failed to bind to address: {}", e);
                }
            },
        }
    }

    pub fn run(self) {
        println!("Server listening on {}", self.addr);
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let _ = std::thread::spawn(|| {
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }
}
