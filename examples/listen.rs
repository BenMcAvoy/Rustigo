use rustigo::prelude::*;

fn main() {
    let mut rustigo = Rustigo::default();

    rustigo.listen("localhost:7878", 4).unwrap();
}
