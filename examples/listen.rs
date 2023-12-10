use rustigo::prelude::*;

fn main() {
    let mut rustigo = Rustigo::default();

    rustigo.listen_and_serve("localhost:7878", 4).unwrap();
}
