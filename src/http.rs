use std::net::TcpStream;
use std::io::{Read, Write};

pub fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream
            .read(&mut buffer)
            .expect("Failed to read from client");

        let readable = String::from_utf8_lossy(&buffer);

        let headers: Vec<&str> = readable.split("\r\n").collect();

        let (_method, path, _version) = {
            let parts: Vec<&str> = headers[0].split(' ').collect();
            (parts[0], parts[1], parts[2])
        };

        println!("Received data from client: {:?}", path);

        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n<h1>Hello, World!</h1>";

        stream
            .write_all(response.as_bytes())
            .expect("Failed to write to client");
    }