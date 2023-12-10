use std::{net::TcpStream, sync::Arc, thread, time::Duration};
use rustigo::prelude::*;

fn index(stream: TcpStream) {
    html!(stream; "<h1>Sleeping</h1>");
    thread::sleep(Duration::from_secs(5));
}

fn main() {
    let mut rustigo = Rustigo::default();

    rustigo.handle("/", Arc::new(index));

    rustigo.listen_and_serve("localhost:7878", 4).unwrap();
}
