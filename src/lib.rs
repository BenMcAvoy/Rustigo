#[macro_use]
mod log;

mod macros;
mod pattern;
mod threadpool;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use pattern::Pattern;
use threadpool::pool::Pool;

pub(crate) type Route = Box<dyn Fn(TcpStream)>;

#[derive(Default)]
pub struct Rustigo {
    routes: HashMap<Pattern, Route>,
}

impl Rustigo {
    pub fn handle(&mut self, path: &str, func: Route) {
        self.routes.insert(Pattern::new(path), func);
    }

    pub fn listen_and_serve(&mut self, address: &str, threads: usize) -> Result<(), String> {
        info!("Listening on http://{address}");

        let listener = TcpListener::bind(address).map_err(|e| e.to_string())?;
        let pool = Pool::new(threads)?;

        for stream in listener.incoming().take(2) {
            let stream = match stream {
                Ok(v) => v,
                Err(e) => {
                    error!(
                        "It seems that there is an error with the TcpStream: {e}.\nQuitting now."
                    );

                    process::exit(1);
                }
            };

            pool.execute(|| match Self::handle_connection(stream) {
                Ok(_) => {}
                Err(e) => {
                    error!(
                        "It seems that there is an error with the TcpStream: {e}.\nQuitting now."
                    );

                    process::exit(1);
                }
            })?;
        }

        error!("Shutting down.");

        Ok(())
    }

    fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().ok_or("No line")??;

        let path = request_line.split(' ').nth(1).ok_or("No path")?;

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
