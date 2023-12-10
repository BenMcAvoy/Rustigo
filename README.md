# Rustigo
A Rust webserver inspired by the Go standard library's HTTPServer

## Features:
- Lightweight
- Pattern matched routes
- No dependencies

## Example
```rust
use rustigo::prelude::*;

fn index(stream: TcpStream, _: Request) {
    html!(stream; "<h1>Hello, world!</h1>");
}

fn main() {
    let mut rustigo = Rustigo::default();

    rustigo.handle("/", Arc::new(index));

    rustigo.listen_and_serve("localhost:7878", 4).unwrap();
}
```
