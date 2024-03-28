//! Rustigo is a simple web server that allows you to handle requests with ease.
//!
//! # Example usage:
//! ```rust
//! use rustigo::prelude::*;
//!
//! fn index(stream: TcpStream, _: Request) {
//!     html!(stream; "<h1>Hello, world!</h1>");
//! }
//!
//! fn main() {
//!     let mut rustigo = Rustigo::default();
//!
//!     rustigo.handle("/", Arc::new(index));
//!
//!     rustigo.listen("localhost:7878", 4).unwrap();
//! }
//! ```

#[macro_use]
mod log;

mod macros;
mod pattern;
mod request;
mod threadpool;
mod traits;

pub mod prelude;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;

use pattern::Pattern;
use request::Request;
use threadpool::pool::Pool;
use traits::IntoArc;

/// A route is a function that takes a TcpStream and a Request and writes a response to the stream.
pub(crate) type Route = Arc<dyn Fn(TcpStream, Request) + Sync + Send>;

/// Rustigo is a simple web server that allows you to handle requests with ease.
/// It's a rust webserver inspired by the Go standard library's HTTPServer.
pub struct Rustigo {
    routes: Arc<Mutex<HashMap<Pattern, Route>>>,
}

/// Get the route that matches the given key (resource).
fn get_route(routes: Arc<Mutex<HashMap<Pattern, Route>>>, key: &str) -> Option<Route> {
    routes
        .lock()
        .unwrap()
        .iter()
        .find_map(|(pattern, route)| pattern.matches(key).then(|| route.clone()))
}

impl Default for Rustigo {
    fn default() -> Self {
        Self::new()
    }
}

impl Rustigo {
    /// Create a new Rustigo instance.
    pub fn new() -> Self {
        let routes = HashMap::default();
        let routes = Arc::new(Mutex::new(routes));

        Self { routes }
    }

    /// Handle a request at the given path.
    /// The path is a string that represents the path of the request.
    /// If a handler is found, it will be a function of type `Route`.
    pub fn handle<T: IntoArc + 'static>(&mut self, path: &str, func: T) -> &mut Self {
        let func = func.into_arc();
        self.routes.lock().unwrap().insert(Pattern::new(path), func);
        self
    }

    /// Listen on the given address.
    /// The amount of `threads` is the number of threads that will be given to the threadpool.
    pub fn listen(&mut self, address: &str, threads: usize) -> Result<(), String> {
        info!("Listening on http://{address}");

        let listener = TcpListener::bind(address).map_err(|e| e.to_string())?;
        let pool = Pool::new(threads)?;

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(v) => v,

                Err(e) => {
                    error!("Failed to handle request: {e}.\nQuitting now.");
                    process::exit(1);
                }
            };

            let routes = Arc::clone(&self.routes);
            pool.execute(move || match Self::handle_connection(stream, routes) {
                Ok(_) => {}
                Err(e) => error!("Error: {e}"),
            })?;
        }

        error!("Shutting down.");

        Ok(())
    }

    /// Handle a connection from a client.
    fn handle_connection(
        mut stream: TcpStream,
        routes: Arc<Mutex<HashMap<Pattern, Route>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let reader = BufReader::new(&mut stream);
        let lines = reader
            .lines()
            .map(|l| l.unwrap())
            .take_while(|l| !l.is_empty())
            .collect();

        let request = Request::new(lines)?;

        match get_route(routes, &request.resource) {
            Some(route) => {
                info!(
                    "200: {}\n           └─ Response succeeded",
                    request.resource
                );

                route(stream, request)
            }

            None => {
                warn!(
                    "404: {}\n           └─ Response failed: 404 Page not found.",
                    request.resource
                );

                html!(stream; "<h1>404 Page not found.</h1>"; 404)
            }
        }

        Ok(())
    }
}
