use async_std::io::{self, prelude::*};
use async_std::task::{Context, Poll};
use routefinder::Captures;
use std::ops::Index;
use std::pin::Pin;
#[cfg(feature = "cookies")]
use crate::cookies::CookieData;
#[cfg(feature = "cookies")]
use crate::http::cookies::Cookie;
use crate::http::format_err;
use crate::http::headers::{self, HeaderName, HeaderValues, ToHeaderValues};
use crate::http::{self, Body, Method, Mime, StatusCode, Url, Version};
use crate::Response;
pin_project_lite::pin_project! {
    #[doc = " An HTTP request."] #[doc = ""] #[doc =
    " The `Request` gives endpoints access to basic information about the incoming"]
    #[doc =
    " request, route parameters, and various ways of accessing the request's body."]
    #[doc = ""] #[doc =
    " Requests also provide *extensions*, a type map primarily used for low-level"] #[doc
    = " communication between middleware and endpoints."] #[derive(Debug)] pub struct
    Request < State > { pub (crate) state : State, #[pin] pub (crate) req :
    http::Request, pub (crate) route_params : Vec < Captures <'static, 'static >>, }
}
impl<State> Request<State> {
    /// Create a new `Request`.
    pub(crate) fn new(
        state: State,
        req: http_types::Request,
        route_params: Vec<Captures<'static, 'static>>,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    /// Access the request's HTTP method.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// use tide::Request;
    ///
    /// let mut app = tide::new();
    /// app.at("/").get(|req: Request<()>| async move {
    ///     assert_eq!(req.method(), http_types::Method::Get);
    ///     Ok("")
    /// });
    /// app.listen("127.0.0.1:8080").await?;
    /// #
    /// # Ok(()) })}
    /// ```
    #[must_use]
    pub fn method(&self) -> Method {
        panic!("STUB: not implemented");
    }
    /// Access the request's full URI method.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// use tide::Request;
    ///
    /// let mut app = tide::new();
    /// app.at("/").get(|req: Request<()>| async move {
    ///     assert_eq!(req.url(), &"/".parse::<tide::http::Url>().unwrap());
    ///     Ok("")
    /// });
    /// app.listen("127.0.0.1:8080").await?;
    /// #
    /// # Ok(()) })}
    /// ```
    #[must_use]
    pub fn url(&self) -> &Url {
        panic!("STUB: not implemented");
    }
    /// Access the request's HTTP version.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// use tide::Request;
    ///
    /// let mut app = tide::new();
    /// app.at("/").get(|req: Request<()>| async move {
    ///     assert_eq!(req.version(), Some(http_types::Version::Http1_1));
    ///     Ok("")
    /// });
    /// app.listen("127.0.0.1:8080").await?;
    /// #
    /// # Ok(()) })}
    /// ```
    #[must_use]
    pub fn version(&self) -> Option<Version> {
        panic!("STUB: not implemented");
    }
    /// Get the peer socket address for the underlying transport, if
    /// that information is available for this request.
    #[must_use]
    pub fn peer_addr(&self) -> Option<&str> {
        panic!("STUB: not implemented");
    }
    /// Get the local socket address for the underlying transport, if
    /// that information is available for this request.
    #[must_use]
    pub fn local_addr(&self) -> Option<&str> {
        panic!("STUB: not implemented");
    }
    /// Get the remote address for this request.
    ///
    /// This is determined in the following priority:
    /// 1. `Forwarded` header `for` key
    /// 2. The first `X-Forwarded-For` header
    /// 3. Peer address of the transport
    #[must_use]
    pub fn remote(&self) -> Option<&str> {
        panic!("STUB: not implemented");
    }
    /// Get the destination host for this request.
    ///
    /// This is determined in the following priority:
    /// 1. `Forwarded` header `host` key
    /// 2. The first `X-Forwarded-Host` header
    /// 3. `Host` header
    /// 4. URL domain, if any
    #[must_use]
    pub fn host(&self) -> Option<&str> {
        panic!("STUB: not implemented");
    }
    /// Get the request content type as a `Mime`.
    ///
    /// This gets the request `Content-Type` header.
    ///
    /// [Read more on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
    #[must_use]
    pub fn content_type(&self) -> Option<Mime> {
        panic!("STUB: not implemented");
    }
    /// Get an HTTP header.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// use tide::Request;
    ///
    /// let mut app = tide::new();
    /// app.at("/").get(|req: Request<()>| async move {
    ///     assert_eq!(req.header("X-Forwarded-For").unwrap(), "127.0.0.1");
    ///     Ok("")
    /// });
    /// app.listen("127.0.0.1:8080").await?;
    /// #
    /// # Ok(()) })}
    /// ```
    #[must_use]
    pub fn header(
        &self,
        key: impl Into<http_types::headers::HeaderName>,
    ) -> Option<&http_types::headers::HeaderValues> {
        panic!("STUB: not implemented");
    }
    /// Get a mutable reference to a header.
    pub fn header_mut(
        &mut self,
        name: impl Into<HeaderName>,
    ) -> Option<&mut HeaderValues> {
        panic!("STUB: not implemented");
    }
    /// Set an HTTP header.
    pub fn insert_header(
        &mut self,
        name: impl Into<HeaderName>,
        values: impl ToHeaderValues,
    ) -> Option<HeaderValues> {
        panic!("STUB: not implemented");
    }
    /// Append a header to the headers.
    ///
    /// Unlike `insert` this function will not override the contents of a header, but insert a
    /// header if there aren't any. Or else append to the existing list of headers.
    pub fn append_header(
        &mut self,
        name: impl Into<HeaderName>,
        values: impl ToHeaderValues,
    ) {
        panic!("STUB: not implemented");
    }
    /// Remove a header.
    pub fn remove_header(
        &mut self,
        name: impl Into<HeaderName>,
    ) -> Option<HeaderValues> {
        panic!("STUB: not implemented");
    }
    /// An iterator visiting all header pairs in arbitrary order.
    #[must_use]
    pub fn iter(&self) -> headers::Iter<'_> {
        panic!("STUB: not implemented");
    }
    /// An iterator visiting all header pairs in arbitrary order, with mutable references to the
    /// values.
    #[must_use]
    pub fn iter_mut(&mut self) -> headers::IterMut<'_> {
        panic!("STUB: not implemented");
    }
    /// An iterator visiting all header names in arbitrary order.
    #[must_use]
    pub fn header_names(&self) -> headers::Names<'_> {
        panic!("STUB: not implemented");
    }
    /// An iterator visiting all header values in arbitrary order.
    #[must_use]
    pub fn header_values(&self) -> headers::Values<'_> {
        panic!("STUB: not implemented");
    }
    /// Get a request extension value.
    #[must_use]
    pub fn ext<T: Send + Sync + 'static>(&self) -> Option<&T> {
        panic!("STUB: not implemented");
    }
    /// Get a mutable reference to value stored in request extensions.
    #[must_use]
    pub fn ext_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        panic!("STUB: not implemented");
    }
    /// Set a request extension value.
    pub fn set_ext<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
        panic!("STUB: not implemented");
    }
    #[must_use]
    ///  Access application scoped state.
    pub fn state(&self) -> &State {
        panic!("STUB: not implemented");
    }
    /// Extract and parse a route parameter by name.
    ///
    /// Returns the parameter as a `&str`, borrowed from this `Request`.
    ///
    /// The name should *not* include the leading `:`.
    ///
    /// # Errors
    ///
    /// An error is returned if `key` is not a valid parameter for the route.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// use tide::{Request, Result};
    ///
    /// async fn greet(req: Request<()>) -> Result<String> {
    ///     let name = req.param("name").unwrap_or("world");
    ///     Ok(format!("Hello, {}!", name))
    /// }
    ///
    /// let mut app = tide::new();
    /// app.at("/hello").get(greet);
    /// app.at("/hello/:name").get(greet);
    /// app.listen("127.0.0.1:8080").await?;
    /// #
    /// # Ok(()) })}
    /// ```
    pub fn param(&self, key: &str) -> crate::Result<&str> {
        panic!("STUB: not implemented");
    }
    /// Fetch the wildcard from the route, if it exists
    ///
    /// Returns the parameter as a `&str`, borrowed from this `Request`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// use tide::{Request, Result};
    ///
    /// async fn greet(req: Request<()>) -> Result<String> {
    ///     let name = req.wildcard().unwrap_or("world");
    ///     Ok(format!("Hello, {}!", name))
    /// }
    ///
    /// let mut app = tide::new();
    /// app.at("/hello/*").get(greet);
    /// app.listen("127.0.0.1:8080").await?;
    /// #
    /// # Ok(()) })}
    /// ```
    pub fn wildcard(&self) -> Option<&str> {
        panic!("STUB: not implemented");
    }
    /// Parse the URL query component into a struct, using [serde_qs](https://docs.rs/serde_qs). To
    /// get the entire query as an unparsed string, use `request.url().query()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use tide::http::{self, convert::Deserialize};
    /// use tide::Request;
    ///
    /// // An owned structure:
    ///
    /// #[derive(Deserialize)]
    /// struct Index {
    ///     page: u32,
    ///     selections: HashMap<String, String>,
    /// }
    ///
    /// let req: Request<()> = http::Request::get("https://httpbin.org/get?page=2&selections[width]=narrow&selections[height]=tall").into();
    /// let Index { page, selections } = req.query().unwrap();
    /// assert_eq!(page, 2);
    /// assert_eq!(selections["width"], "narrow");
    /// assert_eq!(selections["height"], "tall");
    ///
    /// // Using borrows:
    ///
    /// #[derive(Deserialize)]
    /// struct Query<'q> {
    ///     format: &'q str,
    /// }
    ///
    /// let req: Request<()> = http::Request::get("https://httpbin.org/get?format=bananna").into();
    /// let Query { format } = req.query().unwrap();
    /// assert_eq!(format, "bananna");
    /// ```
    pub fn query<'de, T: serde::de::Deserialize<'de>>(&'de self) -> crate::Result<T> {
        panic!("STUB: not implemented");
    }
    /// Set the body reader.
    pub fn set_body(&mut self, body: impl Into<Body>) {
        panic!("STUB: not implemented");
    }
    /// Take the request body as a `Body`.
    ///
    /// This method can be called after the body has already been taken or read,
    /// but will return an empty `Body`.
    ///
    /// This is useful for consuming the body via an AsyncReader or AsyncBufReader.
    pub fn take_body(&mut self) -> Body {
        panic!("STUB: not implemented");
    }
    /// Reads the entire request body into a byte buffer.
    ///
    /// This method can be called after the body has already been read, but will
    /// produce an empty buffer.
    ///
    /// # Errors
    ///
    /// Any I/O error encountered while reading the body is immediately returned
    /// as an `Err`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// use tide::Request;
    ///
    /// let mut app = tide::new();
    /// app.at("/").get(|mut req: Request<()>| async move {
    ///     let _body: Vec<u8> = req.body_bytes().await.unwrap();
    ///     Ok("")
    /// });
    /// app.listen("127.0.0.1:8080").await?;
    /// #
    /// # Ok(()) })}
    /// ```
    pub async fn body_bytes(&mut self) -> crate::Result<Vec<u8>> {
        panic!("STUB: not implemented");
    }
    /// Reads the entire request body into a string.
    ///
    /// This method can be called after the body has already been read, but will
    /// produce an empty buffer.
    ///
    /// # Errors
    ///
    /// Any I/O error encountered while reading the body is immediately returned
    /// as an `Err`.
    ///
    /// If the body cannot be interpreted as valid UTF-8, an `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// use tide::Request;
    ///
    /// let mut app = tide::new();
    /// app.at("/").get(|mut req: Request<()>| async move {
    ///     let _body: String = req.body_string().await.unwrap();
    ///     Ok("")
    /// });
    /// app.listen("127.0.0.1:8080").await?;
    /// #
    /// # Ok(()) })}
    /// ```
    pub async fn body_string(&mut self) -> crate::Result<String> {
        panic!("STUB: not implemented");
    }
    /// Reads and deserialized the entire request body via json.
    ///
    /// # Errors
    ///
    /// Any I/O error encountered while reading the body is immediately returned
    /// as an `Err`.
    ///
    /// If the body cannot be interpreted as valid json for the target type `T`,
    /// an `Err` is returned.
    pub async fn body_json<T: serde::de::DeserializeOwned>(
        &mut self,
    ) -> crate::Result<T> {
        panic!("STUB: not implemented");
    }
    /// Parse the request body as a form.
    ///
    /// ```rust
    /// # fn main() -> Result<(), std::io::Error> { async_std::task::block_on(async {
    /// use tide::prelude::*;
    /// let mut app = tide::new();
    ///
    /// #[derive(Deserialize)]
    /// struct Animal {
    ///   name: String,
    ///   legs: u8
    /// }
    ///
    /// app.at("/").post(|mut req: tide::Request<()>| async move {
    ///     let animal: Animal = req.body_form().await?;
    ///     Ok(format!(
    ///         "hello, {}! i've put in an order for {} shoes",
    ///         animal.name, animal.legs
    ///     ))
    /// });
    ///
    /// # if false {
    /// app.listen("localhost:8000").await?;
    /// # }
    ///
    /// // $ curl localhost:8000/orders/shoes -d "name=chashu&legs=4"
    /// // hello, chashu! i've put in an order for 4 shoes
    ///
    /// // $ curl localhost:8000/orders/shoes -d "name=mary%20millipede&legs=750"
    /// // number too large to fit in target type
    /// # Ok(()) })}
    /// ```
    pub async fn body_form<T: serde::de::DeserializeOwned>(
        &mut self,
    ) -> crate::Result<T> {
        panic!("STUB: not implemented");
    }
    /// returns a `Cookie` by name of the cookie.
    #[cfg(feature = "cookies")]
    #[must_use]
    pub fn cookie(&self, name: &str) -> Option<Cookie<'static>> {
        panic!("STUB: not implemented");
    }
    /// Retrieves a reference to the current session.
    ///
    /// # Panics
    ///
    /// This method will panic if a tide::sessions:SessionMiddleware has not
    /// been run.
    #[cfg(feature = "sessions")]
    pub fn session(&self) -> &crate::sessions::Session {
        panic!("STUB: not implemented");
    }
    /// Retrieves a mutable reference to the current session.
    ///
    /// # Panics
    ///
    /// This method will panic if a tide::sessions:SessionMiddleware has not
    /// been run.
    #[cfg(feature = "sessions")]
    pub fn session_mut(&mut self) -> &mut crate::sessions::Session {
        panic!("STUB: not implemented");
    }
    /// Get the length of the body stream, if it has been set.
    ///
    /// This value is set when passing a fixed-size object as the body. E.g. a string, or a
    /// buffer. Consumers of this API should check this value to decide whether to use `Chunked`
    /// encoding, or set the response length.
    #[must_use]
    pub fn len(&self) -> Option<usize> {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the request has a set body stream length of zero, `false` otherwise.
    #[must_use]
    pub fn is_empty(&self) -> Option<bool> {
        panic!("STUB: not implemented");
    }
}
impl<State> AsRef<http::Request> for Request<State> {
    fn as_ref(&self) -> &http::Request {
        panic!("STUB: not implemented");
    }
}
impl<State> AsMut<http::Request> for Request<State> {
    fn as_mut(&mut self) -> &mut http::Request {
        panic!("STUB: not implemented");
    }
}
impl<State> AsRef<http::Headers> for Request<State> {
    fn as_ref(&self) -> &http::Headers {
        panic!("STUB: not implemented");
    }
}
impl<State> AsMut<http::Headers> for Request<State> {
    fn as_mut(&mut self) -> &mut http::Headers {
        panic!("STUB: not implemented");
    }
}
impl<State> Read for Request<State> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
}
impl<State> From<Request<State>> for http::Request {
    fn from(request: Request<State>) -> http::Request {
        panic!("STUB: not implemented");
    }
}
impl<State: Default> From<http_types::Request> for Request<State> {
    fn from(request: http_types::Request) -> Request<State> {
        panic!("STUB: not implemented");
    }
}
impl<State: Clone + Send + Sync + 'static> From<Request<State>> for Response {
    fn from(mut request: Request<State>) -> Response {
        panic!("STUB: not implemented");
    }
}
impl<State> IntoIterator for Request<State> {
    type Item = (HeaderName, HeaderValues);
    type IntoIter = http_types::headers::IntoIter;
    /// Returns a iterator of references over the remaining items.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        panic!("STUB: not implemented");
    }
}
impl<'a, State> IntoIterator for &'a Request<State> {
    type Item = (&'a HeaderName, &'a HeaderValues);
    type IntoIter = http_types::headers::Iter<'a>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        panic!("STUB: not implemented");
    }
}
impl<'a, State> IntoIterator for &'a mut Request<State> {
    type Item = (&'a HeaderName, &'a mut HeaderValues);
    type IntoIter = http_types::headers::IterMut<'a>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        panic!("STUB: not implemented");
    }
}
impl<State> Index<HeaderName> for Request<State> {
    type Output = HeaderValues;
    /// Returns a reference to the value corresponding to the supplied name.
    ///
    /// # Panics
    ///
    /// Panics if the name is not present in `Request`.
    #[inline]
    fn index(&self, name: HeaderName) -> &HeaderValues {
        panic!("STUB: not implemented");
    }
}
impl<State> Index<&str> for Request<State> {
    type Output = HeaderValues;
    /// Returns a reference to the value corresponding to the supplied name.
    ///
    /// # Panics
    ///
    /// Panics if the name is not present in `Request`.
    #[inline]
    fn index(&self, name: &str) -> &HeaderValues {
        panic!("STUB: not implemented");
    }
}
