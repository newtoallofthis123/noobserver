# NoobServer

Well I wrote this just to learn more about TCP/IP and actually making my own server.

## How to use

```rust
use noobserver::server::Server;

fn main(){
    let server = Server::new(Some("localhost:5000".to_string()));
    server.run();
}
```

This will start a server on localhost:5000. You can also use `None` instead of `Some("localhost:5000".to_string())` to start the server on localhost:5000 automatically.

## What all can it do?

Well for now, it just responds to _ever_ path with a "Hello World" message.
