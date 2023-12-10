use rustigo::prelude::*;

fn books(stream: TcpStream, _: Request) {
    html!(stream; "<h1>1, 2, 3</h1>");
}

fn book(stream: TcpStream, request: Request) {
    let book: i32 = request
        .resource
        .split("/book/")
        .collect::<String>()
        .parse()
        .unwrap();

    if !(1..4).contains(&book) {
        html!(stream; "<p>That book doesn't exist.<p>");
        return;
    }

    let text = format!("<p>Loading book {book}</p>");

    html!(stream; text);
}

fn main() {
    let mut rustigo = Rustigo::default();

    rustigo.handle("/", books).handle("/book*", book);

    rustigo.listen("localhost:7878", 4).unwrap();
}
