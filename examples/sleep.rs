use rustigo::prelude::*;
use std::time::Duration;
use std::thread;

fn index(stream: TcpStream, _: Request) {
    html!(stream; "<h1>Sleeping</h1>");
    thread::sleep(Duration::from_secs(5));
}

fn main() {
    let mut rustigo = Rustigo::default();

    rustigo.handle("/", Arc::new(index));

    rustigo.listen_and_serve("localhost:7878", 4).unwrap();
}
