use crate::request::Request;

use std::net::TcpStream;
use std::sync::Arc;

/// A trait that allows you to convert a function into an Arc.
/// This is useful for storing routes in a HashMap as they can be turned into Arcs
/// automatically.
pub trait IntoArc {
    fn into_arc(self) -> Arc<dyn Fn(TcpStream, Request) + Send + Sync + 'static>;
}

impl<F: Fn(TcpStream, Request) + Send + Sync + 'static> IntoArc for F {
    fn into_arc(self) -> Arc<dyn Fn(TcpStream, Request) + Send + Sync + 'static> {
        Arc::new(self)
    }
}

impl<F: Fn(TcpStream, Request) + Send + Sync + 'static> IntoArc for Arc<F> {
    fn into_arc(self) -> Arc<dyn Fn(TcpStream, Request) + Send + Sync + 'static> {
        self
    }
}
