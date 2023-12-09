mod macros;
mod threadpool;

use std::io::prelude::*;
use std::io::BufReader;
use std::process;

pub struct Rustigo;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use threadpool::pool::Pool;

impl Rustigo {
    pub fn listen_and_serve(&mut self, threads: usize) -> Result<(), String> {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        let pool = Pool::new(threads)?;

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(v) => v,
                Err(e) => {
                    eprintln!(
                        "It seems that there is an error with the TcpStream: {e}.\nQuitting now."
                    );

                    process::exit(1);
                }
            };

            pool.execute(|| match Self::handle_connection(stream) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                        "It seems that there is an error with the TcpStream: {e}.\nQuitting now."
                    );

                    process::exit(1);
                }
            });
        }

        Ok(())
    }

    fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().ok_or("No line")??;

        let path = request_line.split(' ').nth(1).ok_or("No path")?;

        dbg!(&path);

        match path {
            "/" => html!(stream; "<h1>Hello from root</h1>"),

            "/sleep" => {
                thread::sleep(Duration::from_secs(5));
                html!(stream; "<h1>Hello from root</h1>")
            }

            _ => html!(stream; "<h1>404 Page not found</h1>"; 404),
        }

        Ok(())
    }
}
