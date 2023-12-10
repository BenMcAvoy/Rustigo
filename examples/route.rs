use std::{net::TcpStream, sync::Arc};

use rustigo::{html, Rustigo};

fn index(stream: TcpStream) {
    html!(stream; "<h1>Hello, world!</h1>");
}

fn main() {
    let mut rustigo = Rustigo::default();

    rustigo.handle("/", Arc::new(index));

    rustigo.listen_and_serve("localhost:7878", 4).unwrap();
}
