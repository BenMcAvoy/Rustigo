#[macro_use]
mod log;

mod macros;
mod traits;
mod pattern;
mod request;
mod threadpool;

pub mod prelude;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;

use traits::IntoArc;
use pattern::Pattern;
use request::Request;
use threadpool::pool::Pool;

pub(crate) type Route = Arc<dyn Fn(TcpStream, Request) + Sync + Send>;

pub struct Rustigo {
    routes: Arc<Mutex<HashMap<Pattern, Route>>>,
}

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
    pub fn new() -> Self {
        let routes = HashMap::default();
        let routes = Arc::new(Mutex::new(routes));

        Self { routes }
    }

    pub fn handle<T: IntoArc + 'static>(&mut self, path: &str, func: T) -> &mut Self {
        let func = func.into_arc();
        self.routes.lock().unwrap().insert(Pattern::new(path), func);
        self
    }

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

        info!("Resource: {}\n           └─ Responding", request.resource);

        match get_route(routes, &request.resource) {
            Some(route) => route(stream, request),
            None => html!(stream; "<h1>404 Page not found.</h1>"; 404),
        }

        Ok(())
    }
}
