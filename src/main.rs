mod server;
mod http;

fn main() {
    let server = server::Server::new(Some("localhost:5000".to_string()));
    server.run();
}
