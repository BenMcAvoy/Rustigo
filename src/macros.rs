/// A macro to return html content to a TCPStream.
/// Example usage:
/// ```rust
/// html!(stream; "<h1>Hello, World!</h1>");
/// ```
#[macro_export]
macro_rules! html {
    ($stream:expr; $body:expr) => {{
        use std::io::Write;

        let mut stream = $stream.try_clone().unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            $body.len(),
            $body
        );
        stream.write_all(response.as_bytes()).unwrap();
    }};
    ($stream:expr; $body:expr; $status:expr) => {{
        use std::io::Write;

        let mut stream = $stream.try_clone().unwrap();
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            $status,
            $body.len(),
            $body
        );
        stream.write_all(response.as_bytes()).unwrap();
    }};
}

/// A macro to return text content to a TCPStream.
/// Example usage:
/// ```rust
/// text!(stream; "https://cdn.mysite.com/custom_generated_link_here");
/// ```
#[macro_export]
macro_rules! text {
    ($stream:expr; $body:expr) => {{
        use std::io::Write;

        let mut stream = $stream.try_clone().unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            $body.len(),
            $body
        );
        stream.write_all(response.as_bytes()).unwrap();
    }};
    ($stream:expr; $body:expr; $status:expr) => {{
        use std::io::Write;

        let mut stream = $stream.try_clone().unwrap();
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            $status,
            $body.len(),
            $body
        );
        stream.write_all(response.as_bytes()).unwrap();
    }};
}
