use crate::listener::{Listener, ToListener};
use crate::Server;
use std::fmt::{self, Debug, Display, Formatter};
use async_std::io;
use kv_log_macro::info;
use crate::listener::ListenInfo;
/// FailoverListener allows tide to attempt to listen in a sequential
/// order to any number of ports/addresses. The first successful
/// listener is used.
///
/// # Example:
/// ```rust
/// fn main() -> Result<(), std::io::Error> {
///    async_std::task::block_on(async {
///        let mut app = tide::new();
///        app.at("/").get(|_| async { Ok("Hello, world!") });
///
///        let mut listener = tide::listener::FailoverListener::new();
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
pub struct FailoverListener<State> {
    listeners: Vec<Option<Box<dyn Listener<State>>>>,
    index: Option<usize>,
}
impl<State> FailoverListener<State>
where
    State: Clone + Send + Sync + 'static,
{
    /// creates a new FailoverListener
    pub fn new() -> Self {
        panic!("STUB: not implemented");
    }
    /// Adds any [`ToListener`](crate::listener::ToListener) to this
    /// FailoverListener. An error result represents a failure to convert
    /// the [`ToListener`](crate::listener::ToListener) into a
    /// [`Listener`](crate::listener::Listener).
    ///
    /// ```rust
    /// # fn main() -> std::io::Result<()> {
    /// let mut listener = tide::listener::FailoverListener::new();
    /// listener.add("127.0.0.1:8000")?;
    /// listener.add(("localhost", 8001))?;
    /// # if cfg!(unix) {
    /// listener.add("http+unix:///var/run/tide")?;
    /// # }
    /// # std::mem::drop(tide::new().listen(listener)); // for the State generic
    /// # Ok(()) }
    /// ```
    pub fn add<L>(&mut self, listener: L) -> io::Result<()>
    where
        L: ToListener<State>,
    {
        panic!("STUB: not implemented");
    }
    /// `FailoverListener::with_listener` allows for chained construction of a FailoverListener:
    /// ```rust,no_run
    /// # use tide::listener::FailoverListener;
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async move {
    /// # let app = tide::new();
    /// app.listen(
    ///     FailoverListener::new()
    ///         .with_listener("127.0.0.1:8080")
    ///         .with_listener(("localhost", 8081)),
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
impl<State> Listener<State> for FailoverListener<State>
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
impl<State> Debug for FailoverListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<State> Display for FailoverListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
