use rustigo::Rustigo;

fn main() {
    let mut rustigo = Rustigo;

    rustigo.listen_and_serve("localhost:7878", 4).unwrap();
}
