use super::{is_transient_error, ListenInfo};
use crate::listener::Listener;
use crate::Server;
use std::fmt::{self, Display, Formatter};
use async_std::net::{self, SocketAddr, TcpStream};
use async_std::prelude::*;
use async_std::{io, task};
use kv_log_macro::error;
/// This represents a tide [Listener](crate::listener::Listener) that
/// wraps an [async_std::net::TcpListener]. It is implemented as an
/// enum in order to allow creation of a tide::listener::TcpListener
/// from a SocketAddr spec that has not yet been bound OR from a bound
/// TcpListener.
///
/// This is currently crate-visible only, and tide users are expected
/// to create these through [ToListener](crate::ToListener) conversions.
pub struct TcpListener<State> {
    addrs: Option<Vec<SocketAddr>>,
    listener: Option<net::TcpListener>,
    server: Option<Server<State>>,
    info: Option<ListenInfo>,
}
impl<State> TcpListener<State> {
    pub fn from_addrs(addrs: Vec<SocketAddr>) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn from_listener(tcp_listener: impl Into<net::TcpListener>) -> Self {
        panic!("STUB: not implemented");
    }
}
fn handle_tcp<State: Clone + Send + Sync + 'static>(
    app: Server<State>,
    stream: TcpStream,
) {
    panic!("STUB: not implemented");
}
#[async_trait::async_trait]
impl<State> Listener<State> for TcpListener<State>
where
    State: Clone + Send + Sync + 'static,
{
    async fn bind(&mut self, server: Server<State>) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    async fn accept(&mut self) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    fn info(&self) -> Vec<ListenInfo> {
        panic!("STUB: not implemented");
    }
}
impl<State> fmt::Debug for TcpListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<State> Display for TcpListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
