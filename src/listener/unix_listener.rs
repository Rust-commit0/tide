use super::{is_transient_error, ListenInfo};
use crate::listener::Listener;
use crate::Server;
use std::fmt::{self, Display, Formatter};
use async_std::os::unix::net::{self, SocketAddr, UnixStream};
use async_std::path::PathBuf;
use async_std::prelude::*;
use async_std::{io, task};
use kv_log_macro::error;
/// This represents a tide [Listener](crate::listener::Listener) that
/// wraps an [async_std::os::unix::net::UnixListener]. It is implemented as an
/// enum in order to allow creation of a tide::listener::UnixListener
/// from a [`PathBuf`] spec that has not yet been bound OR from a bound
/// [async_std::os::unix::net::UnixListener].
///
/// This is currently crate-visible only, and tide users are expected
/// to create these through [ToListener](crate::ToListener) conversions.
pub struct UnixListener<State> {
    path: Option<PathBuf>,
    listener: Option<net::UnixListener>,
    server: Option<Server<State>>,
    info: Option<ListenInfo>,
}
impl<State> UnixListener<State> {
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn from_listener(unix_listener: impl Into<net::UnixListener>) -> Self {
        panic!("STUB: not implemented");
    }
}
fn handle_unix<State: Clone + Send + Sync + 'static>(
    app: Server<State>,
    stream: UnixStream,
) {
    panic!("STUB: not implemented");
}
#[async_trait::async_trait]
impl<State> Listener<State> for UnixListener<State>
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
impl<State> fmt::Debug for UnixListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<State> Display for UnixListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
fn unix_socket_addr_to_string(result: io::Result<SocketAddr>) -> Option<String> {
    panic!("STUB: not implemented");
}
