use rustigo::prelude::*;
use std::thread;
use std::time::Duration;

fn index(stream: TcpStream, _: Request) {
    html!(stream; "<h1>Sleeping</h1>");
    thread::sleep(Duration::from_secs(5));
}

fn main() {
    let mut rustigo = Rustigo::default();

    rustigo.handle("/", index);

    rustigo.listen_and_serve("localhost:7878", 4).unwrap();
}
