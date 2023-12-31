use rustigo::prelude::*;

fn index(stream: TcpStream, _: Request) {
    html!(stream; "<h1>Hello, world!</h1>");
}

fn main() {
    let mut rustigo = Rustigo::default();

    rustigo.handle("/", index);

    rustigo.listen("localhost:7878", 4).unwrap();
}
