#[cfg(unix)]
use super::UnixListener;
use super::{ListenInfo, Listener, TcpListener};
use crate::Server;
use async_std::io;
use std::fmt::{self, Debug, Display, Formatter};
/// This is an enum that contains variants for each of the listeners
/// that can be parsed from a string. This is used as the associated
/// Listener type for the string-parsing
/// [ToListener](crate::listener::ToListener) implementations
///
/// This is currently crate-visible only, and tide users are expected
/// to create these through [ToListener](crate::ToListener) conversions.
pub enum ParsedListener<State> {
    #[cfg(unix)]
    Unix(UnixListener<State>),
    Tcp(TcpListener<State>),
}
impl<State> Debug for ParsedListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<State> Display for ParsedListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
#[async_trait::async_trait]
impl<State> Listener<State> for ParsedListener<State>
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
