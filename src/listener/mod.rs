//! Types that represent HTTP transports and binding
mod concurrent_listener;
mod failover_listener;
#[cfg(feature = "h1-server")]
mod parsed_listener;
#[cfg(feature = "h1-server")]
mod tcp_listener;
mod to_listener;
#[cfg(feature = "h1-server")]
mod to_listener_impls;
#[cfg(all(unix, feature = "h1-server"))]
mod unix_listener;
use std::fmt::{Debug, Display};
use async_std::io;
use async_trait::async_trait;
use crate::Server;
pub use concurrent_listener::ConcurrentListener;
pub use failover_listener::FailoverListener;
pub use to_listener::ToListener;
#[cfg(feature = "h1-server")]
pub(crate) use parsed_listener::ParsedListener;
#[cfg(feature = "h1-server")]
pub(crate) use tcp_listener::TcpListener;
#[cfg(all(unix, feature = "h1-server"))]
pub(crate) use unix_listener::UnixListener;
/// The Listener trait represents an implementation of http transport for a tide
/// application. In order to provide a Listener to tide, you will also need to
/// implement at least one [`ToListener`](crate::listener::ToListener) that
/// outputs your Listener type.
#[async_trait]
pub trait Listener<State>: Debug + Display + Send + 'static
where
    State: Send + Sync + 'static,
{
    /// Bind the listener. This starts the listening process by opening the
    /// necessary network ports, but not yet accepting incoming connections. This
    /// method must be called before `accept`.
    async fn bind(&mut self, app: Server<State>) -> io::Result<()>;
    /// Start accepting incoming connections. This method must be called only
    /// after `bind` has succeeded.
    async fn accept(&mut self) -> io::Result<()>;
    /// Expose information about the connection. This should always return valid
    /// data after `bind` has succeeded.
    fn info(&self) -> Vec<ListenInfo>;
}
#[async_trait]
impl<L, State> Listener<State> for Box<L>
where
    L: Listener<State>,
    State: Send + Sync + 'static,
{
    async fn bind(&mut self, app: Server<State>) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    async fn accept(&mut self) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    fn info(&self) -> Vec<ListenInfo> {
        panic!("STUB: not implemented");
    }
}
/// crate-internal shared logic used by tcp and unix listeners to
/// determine if an io::Error needs a backoff delay. Transient error
/// types do not require a delay.
#[cfg(feature = "h1-server")]
pub(crate) fn is_transient_error(e: &io::Error) -> bool {
    panic!("STUB: not implemented");
}
/// Information about the `Listener`.
///
/// See [`Report`](../listener/trait.Report.html) for more.
#[derive(Debug, Clone)]
pub struct ListenInfo {
    conn_string: String,
    transport: String,
    tls: bool,
}
impl ListenInfo {
    /// Create a new instance of `ListenInfo`.
    ///
    /// This method should only be called when implementing a new Tide `listener`
    /// strategy.
    pub fn new(conn_string: String, transport: String, tls: bool) -> Self {
        panic!("STUB: not implemented");
    }
    /// Get the connection string.
    pub fn connection(&self) -> &str {
        panic!("STUB: not implemented");
    }
    /// The underlying transport this connection listens on.
    ///
    /// Examples are: "tcp", "uds", etc.
    pub fn transport(&self) -> &str {
        panic!("STUB: not implemented");
    }
    /// Is the connection encrypted?
    pub fn is_encrypted(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
impl Display for ListenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("STUB: not implemented");
    }
}
