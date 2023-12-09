use rustigo::Rustigo;

fn main() {
    let mut rustigo = Rustigo;

    rustigo.listen_and_serve(4).unwrap();
}
