use crate::listener::{ListenInfo, Listener, ToListener};
use crate::Server;
use std::fmt::{self, Debug, Display, Formatter};
use async_std::io;
use futures_util::stream::{futures_unordered::FuturesUnordered, StreamExt};
/// ConcurrentListener allows tide to listen on any number of transports
/// simultaneously (such as tcp ports, unix sockets, or tls).
///
/// # Example:
/// ```rust
/// fn main() -> Result<(), std::io::Error> {
///    async_std::task::block_on(async {
///        let mut app = tide::new();
///        app.at("/").get(|_| async { Ok("Hello, world!") });
///
///        let mut listener = tide::listener::ConcurrentListener::new();
///        listener.add("127.0.0.1:8000")?;
///        listener.add(async_std::net::TcpListener::bind("127.0.0.1:8001").await?)?;
/// # if cfg!(unix) {
///        listener.add("http+unix://unix.socket")?;
/// # }
///
/// # if false {
///        app.listen(listener).await?;
/// # }
///        Ok(())
///    })
///}
///```
#[derive(Default)]
pub struct ConcurrentListener<State> {
    listeners: Vec<Box<dyn Listener<State>>>,
}
impl<State: Clone + Send + Sync + 'static> ConcurrentListener<State> {
    /// creates a new ConcurrentListener
    pub fn new() -> Self {
        panic!("STUB: not implemented");
    }
    /// Adds any [`ToListener`](crate::listener::ToListener) to this
    /// ConcurrentListener. An error result represents a failure to convert
    /// the [`ToListener`](crate::listener::ToListener) into a
    /// [`Listener`](crate::listener::Listener).
    ///
    /// ```rust
    /// # fn main() -> std::io::Result<()> {
    /// let mut listener = tide::listener::ConcurrentListener::new();
    /// listener.add("127.0.0.1:8000")?;
    /// listener.add(("localhost", 8001))?;
    /// listener.add(std::net::TcpListener::bind(("localhost", 8002))?)?;
    /// # std::mem::drop(tide::new().listen(listener)); // for the State generic
    /// # Ok(()) }
    /// ```
    pub fn add<L>(&mut self, listener: L) -> io::Result<()>
    where
        L: ToListener<State>,
    {
        panic!("STUB: not implemented");
    }
    /// `ConcurrentListener::with_listener` allows for chained construction of a ConcurrentListener:
    /// ```rust,no_run
    /// # use tide::listener::ConcurrentListener;
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async move {
    /// # let app = tide::new();
    /// app.listen(
    ///     ConcurrentListener::new()
    ///         .with_listener("127.0.0.1:8080")
    ///         .with_listener(async_std::net::TcpListener::bind("127.0.0.1:8081").await?),
    /// ).await?;
    /// #  Ok(()) }) }
    pub fn with_listener<L>(mut self, listener: L) -> Self
    where
        L: ToListener<State>,
    {
        panic!("STUB: not implemented");
    }
}
#[async_trait::async_trait]
impl<State> Listener<State> for ConcurrentListener<State>
where
    State: Clone + Send + Sync + 'static,
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
impl<State> Debug for ConcurrentListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<State> Display for ConcurrentListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
