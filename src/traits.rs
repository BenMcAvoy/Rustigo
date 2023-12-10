use crate::request::Request;
use std::net::TcpStream;
use std::sync::Arc;

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
