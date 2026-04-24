use std::fmt::Debug;
use std::io;
use std::path::Path;
use std::sync::Arc;
use crate::endpoint::MiddlewareEndpoint;
use crate::fs::{ServeDir, ServeFile};
use crate::{router::Router, Endpoint, Middleware};
use kv_log_macro::trace;
/// A handle to a route.
///
/// All HTTP requests are made against resources. After using [`Server::at`] (or
/// [`Route::at`]) to establish a route, the `Route` type can be used to
/// establish endpoints for various HTTP methods at that path. Also, using
/// `nest`, it can be used to set up a subrouter.
///
/// [`Server::at`]: ./struct.Server.html#method.at
#[allow(missing_debug_implementations)]
pub struct Route<'a, State> {
    router: &'a mut Router<State>,
    path: String,
    middleware: Vec<Arc<dyn Middleware<State>>>,
    /// Indicates whether the path of current route is treated as a prefix. Set by
    /// [`strip_prefix`].
    ///
    /// [`strip_prefix`]: #method.strip_prefix
    prefix: bool,
}
impl<'a, State: Clone + Send + Sync + 'static> Route<'a, State> {
    pub(crate) fn new(router: &'a mut Router<State>, path: String) -> Route<'a, State> {
        panic!("STUB: not implemented");
    }
    /// Extend the route with the given `path`.
    pub fn at<'b>(&'b mut self, path: &str) -> Route<'b, State> {
        panic!("STUB: not implemented");
    }
    /// Get the current path.
    #[must_use]
    pub fn path(&self) -> &str {
        panic!("STUB: not implemented");
    }
    /// Treat the current path as a prefix, and strip prefixes from requests.
    ///
    /// This method is marked unstable as its name might change in the near future.
    ///
    /// Endpoints will be given a path with the prefix removed.
    #[cfg(any(feature = "unstable", feature = "docs"))]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    pub fn strip_prefix(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Apply the given middleware to the current route.
    pub fn with<M>(&mut self, middleware: M) -> &mut Self
    where
        M: Middleware<State>,
    {
        panic!("STUB: not implemented");
    }
    /// Reset the middleware chain for the current route, if any.
    pub fn reset_middleware(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Nest a [`Server`] at the current path.
    ///
    /// # Note
    ///
    /// The outer server *always* has precedence when disambiguating
    /// overlapping paths. For example in the following example `/hello` will
    /// return "Unexpected" to the client
    ///
    /// ```no_run
    /// #[async_std::main]
    /// async fn main() -> Result<(), std::io::Error> {
    ///     let mut app = tide::new();
    ///     app.at("/hello").nest({
    ///         let mut example = tide::with_state("world");
    ///         example
    ///             .at("/")
    ///             .get(|req: tide::Request<&'static str>| async move {
    ///                 Ok(format!("Hello {state}!", state = req.state()))
    ///             });
    ///         example
    ///     });
    ///     app.at("/*").get(|_| async { Ok("Unexpected") });
    ///     app.listen("127.0.0.1:8080").await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Server`]: struct.Server.html
    pub fn nest<InnerState>(&mut self, service: crate::Server<InnerState>) -> &mut Self
    where
        State: Clone + Send + Sync + 'static,
        InnerState: Clone + Send + Sync + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Serve a directory statically.
    ///
    /// Each file will be streamed from disk, and a mime type will be determined
    /// based on magic bytes.
    ///
    /// # Security
    ///
    /// This handler ensures no folders outside the specified folder can be
    /// served, and attempts to access any path outside this folder (no matter
    /// if it exists or not) will return `StatusCode::Forbidden` to the caller.
    ///
    /// # Examples
    ///
    /// Serve the contents of the local directory `./public/images/*` from
    /// `localhost:8080/images/*`.
    ///
    /// ```no_run
    /// #[async_std::main]
    /// async fn main() -> Result<(), std::io::Error> {
    ///     let mut app = tide::new();
    ///     app.at("/images/*").serve_dir("public/images/")?;
    ///     app.listen("127.0.0.1:8080").await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn serve_dir(&mut self, dir: impl AsRef<Path>) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    /// Serve a static file.
    ///
    /// The file will be streamed from disk, and a mime type will be determined
    /// based on magic bytes. Similar to serve_dir
    pub fn serve_file(&mut self, file: impl AsRef<Path>) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    /// Add an endpoint for the given HTTP method
    pub fn method(
        &mut self,
        method: http_types::Method,
        ep: impl Endpoint<State>,
    ) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Add an endpoint for all HTTP methods, as a fallback.
    ///
    /// Routes with specific HTTP methods will be tried first.
    pub fn all(&mut self, ep: impl Endpoint<State>) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Add an endpoint for `GET` requests
    pub fn get(&mut self, ep: impl Endpoint<State>) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Add an endpoint for `HEAD` requests
    pub fn head(&mut self, ep: impl Endpoint<State>) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Add an endpoint for `PUT` requests
    pub fn put(&mut self, ep: impl Endpoint<State>) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Add an endpoint for `POST` requests
    pub fn post(&mut self, ep: impl Endpoint<State>) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Add an endpoint for `DELETE` requests
    pub fn delete(&mut self, ep: impl Endpoint<State>) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Add an endpoint for `OPTIONS` requests
    pub fn options(&mut self, ep: impl Endpoint<State>) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Add an endpoint for `CONNECT` requests
    pub fn connect(&mut self, ep: impl Endpoint<State>) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Add an endpoint for `PATCH` requests
    pub fn patch(&mut self, ep: impl Endpoint<State>) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Add an endpoint for `TRACE` requests
    pub fn trace(&mut self, ep: impl Endpoint<State>) -> &mut Self {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug)]
struct StripPrefixEndpoint<E>(std::sync::Arc<E>);
impl<E> StripPrefixEndpoint<E> {
    fn new(ep: E) -> Self {
        panic!("STUB: not implemented");
    }
}
impl<E> Clone for StripPrefixEndpoint<E> {
    fn clone(&self) -> Self {
        panic!("STUB: not implemented");
    }
}
#[async_trait::async_trait]
impl<State, E> Endpoint<State> for StripPrefixEndpoint<E>
where
    State: Clone + Send + Sync + 'static,
    E: Endpoint<State>,
{
    async fn call(&self, req: crate::Request<State>) -> crate::Result {
        panic!("STUB: not implemented");
    }
}
