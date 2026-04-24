use std::convert::TryInto;
use std::fmt::{Debug, Display};
use std::ops::Index;
use serde::Serialize;
#[cfg(feature = "cookies")]
use crate::http::cookies::Cookie;
use crate::http::headers::{self, HeaderName, HeaderValues, ToHeaderValues};
use crate::http::{self, Body, Error, Mime, StatusCode};
use crate::ResponseBuilder;
#[cfg(feature = "cookies")]
#[derive(Debug)]
pub(crate) enum CookieEvent {
    Added(Cookie<'static>),
    Removed(Cookie<'static>),
}
/// An HTTP response
#[derive(Debug)]
pub struct Response {
    pub(crate) res: http::Response,
    pub(crate) error: Option<Error>,
    #[cfg(feature = "cookies")]
    pub(crate) cookie_events: Vec<CookieEvent>,
}
impl Response {
    /// Create a new instance.
    #[must_use]
    pub fn new<S>(status: S) -> Self
    where
        S: TryInto<StatusCode>,
        S::Error: Debug,
    {
        panic!("STUB: not implemented");
    }
    /// Begin a chained response builder. For more details, see [ResponseBuilder](crate::ResponseBuilder)
    ///
    /// # Example:
    /// ```rust
    /// # use tide::{StatusCode, Response, http::mime};
    /// # async_std::task::block_on(async move {
    /// let mut response = Response::builder(203)
    ///     .body("<html>hi</html>")
    ///     .header("custom-header", "value")
    ///     .content_type(mime::HTML)
    ///     .build();
    ///
    /// assert_eq!(response.take_body().into_string().await.unwrap(), "<html>hi</html>");
    /// assert_eq!(response.status(), StatusCode::NonAuthoritativeInformation);
    /// assert_eq!(response["custom-header"], "value");
    /// assert_eq!(response["content-type"], "text/html;charset=utf-8");
    /// # });
    /// ```
    #[must_use]
    pub fn builder<S>(status: S) -> ResponseBuilder
    where
        S: TryInto<StatusCode>,
        S::Error: Debug,
    {
        panic!("STUB: not implemented");
    }
    /// Returns the http status code.
    #[must_use]
    pub fn status(&self) -> crate::StatusCode {
        panic!("STUB: not implemented");
    }
    /// Set the http status code.
    ///
    /// # Example:
    /// ```rust
    /// # use tide::{StatusCode, Response};
    /// let mut response = Response::new(StatusCode::Ok);
    ///
    /// response.set_status(418); // the status can be a valid u16 http status code
    /// assert_eq!(response.status(), StatusCode::ImATeapot);
    ///
    /// response.set_status(StatusCode::NonAuthoritativeInformation); // or a tide::StatusCode
    /// assert_eq!(response.status(), StatusCode::NonAuthoritativeInformation);
    /// ```
    /// # Panics
    /// `set_status` will panic if the status argument cannot be successfully converted into a StatusCode.
    ///
    /// ```should_panic
    /// # use tide::Response;
    /// Response::new(200).set_status(210); // this is not an established status code and will panic
    /// ```
    pub fn set_status<S>(&mut self, status: S)
    where
        S: TryInto<StatusCode>,
        S::Error: Debug,
    {
        panic!("STUB: not implemented");
    }
    /// Get the length of the body.
    #[must_use]
    pub fn len(&self) -> Option<usize> {
        panic!("STUB: not implemented");
    }
    /// Checks if the body is empty.
    #[must_use]
    pub fn is_empty(&self) -> Option<bool> {
        panic!("STUB: not implemented");
    }
    /// Get an HTTP header.
    #[must_use]
    pub fn header(&self, name: impl Into<HeaderName>) -> Option<&HeaderValues> {
        panic!("STUB: not implemented");
    }
    /// Get an HTTP header mutably.
    #[must_use]
    pub fn header_mut(
        &mut self,
        name: impl Into<HeaderName>,
    ) -> Option<&mut HeaderValues> {
        panic!("STUB: not implemented");
    }
    /// Remove a header.
    pub fn remove_header(
        &mut self,
        name: impl Into<HeaderName>,
    ) -> Option<HeaderValues> {
        panic!("STUB: not implemented");
    }
    /// Insert an HTTP header.
    pub fn insert_header(
        &mut self,
        key: impl Into<HeaderName>,
        value: impl ToHeaderValues,
    ) {
        panic!("STUB: not implemented");
    }
    /// Append an HTTP header.
    pub fn append_header(
        &mut self,
        key: impl Into<HeaderName>,
        value: impl ToHeaderValues,
    ) {
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
    /// Get the response content type as a `Mime`.
    ///
    /// This gets the request `Content-Type` header.
    ///
    /// [Read more on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
    #[must_use]
    pub fn content_type(&self) -> Option<Mime> {
        panic!("STUB: not implemented");
    }
    /// Set the response content type from a `MIME`.
    ///
    /// This sets the response `Content-Type` header.
    ///
    /// [Read more on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
    pub fn set_content_type(&mut self, mime: impl Into<Mime>) {
        panic!("STUB: not implemented");
    }
    /// Set the body reader.
    pub fn set_body(&mut self, body: impl Into<Body>) {
        panic!("STUB: not implemented");
    }
    /// Take the response body as a `Body`.
    ///
    /// This method can be called after the body has already been taken or read,
    /// but will return an empty `Body`.
    ///
    /// Useful for adjusting the whole body, such as in middleware.
    pub fn take_body(&mut self) -> Body {
        panic!("STUB: not implemented");
    }
    /// Swaps the value of the body with another body, without deinitializing
    /// either one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use async_std::io::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # async_std::task::block_on(async {
    /// #
    /// use tide::Response;
    ///
    /// let mut req = Response::new(200);
    /// req.set_body("Hello, Nori!");
    ///
    /// let mut body = "Hello, Chashu!".into();
    /// req.swap_body(&mut body);
    ///
    /// let mut string = String::new();
    /// body.read_to_string(&mut string).await?;
    /// assert_eq!(&string, "Hello, Nori!");
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn swap_body(&mut self, body: &mut Body) {
        panic!("STUB: not implemented");
    }
    /// Pass JSON as the response body.
    ///
    /// # Mime
    ///
    /// The `Content-Type` is set to `application/json`.
    ///
    /// # Errors
    ///
    /// This method will return an error if the provided data could not be serialized to JSON.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serde::{Deserialize, Serialize};
    /// # use tide::Response;
    /// # #[async_std::main]
    /// # async fn main() -> tide::Result<()> {
    /// #[derive(Deserialize, Serialize)]
    /// struct Ip {
    ///     ip: String
    /// }
    ///
    /// let data = &Ip { ip: "129.0.0.1".into() };
    /// let mut res = Response::new(200);
    /// res.body_json(data)?;
    /// # Ok(()) }
    /// ```
    pub fn body_json(&mut self, json: &impl Serialize) -> crate::Result<()> {
        panic!("STUB: not implemented");
    }
    /// Pass a string as the response body.
    ///
    /// # Mime
    ///
    /// The `Content-Type` is set to `text/plain; charset=utf-8`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tide::Response;
    /// # #[async_std::main]
    /// # async fn main() -> tide::Result<()> {
    /// let data = "hello world".to_string();
    /// let mut res = Response::new(200);
    /// res.body_string(data);
    /// # Ok(()) }
    /// ```
    pub fn body_string(&mut self, string: String) {
        panic!("STUB: not implemented");
    }
    /// Pass bytes as the request body.
    ///
    /// # Mime
    ///
    /// The `Content-Type` is set to `application/octet-stream`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tide::Response;
    /// # #[async_std::main]
    /// # async fn main() -> tide::Result<()> {
    /// let data = b"hello world".to_owned();
    /// let mut res = Response::new(200);
    /// res.body_bytes(data);
    /// # Ok(()) }
    /// ```
    pub fn body_bytes(&mut self, bytes: impl AsRef<[u8]>) {
        panic!("STUB: not implemented");
    }
    /// Pass a file as the response body.
    ///
    /// # Mime
    ///
    /// The `Content-Type` is set based on the file extension using [`mime_guess`] if the operation was
    /// successful. If `path` has no extension, or its extension has no known MIME type mapping,
    /// then `None` is returned.
    ///
    /// [`mime_guess`]: https://docs.rs/mime_guess
    ///
    /// # Errors
    ///
    /// This method will return an error if the file couldn't be read.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tide::Response;
    /// # #[async_std::main]
    /// # async fn main() -> tide::Result<()> {
    /// let mut res = Response::new(200);
    /// res.body_file("./archive.tgz").await?;
    /// # Ok(()) }
    /// ```
    pub async fn body_file(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> std::io::Result<()> {
        panic!("STUB: not implemented");
    }
    /// Insert cookie in the cookie jar.
    #[cfg(feature = "cookies")]
    pub fn insert_cookie(&mut self, cookie: Cookie<'static>) {
        panic!("STUB: not implemented");
    }
    /// Removes the cookie. This instructs the `CookiesMiddleware` to send a cookie with empty value
    /// in the response.
    ///
    /// ## Warning
    /// Take care when calling this function with a cookie that was returned by
    /// [`Request::cookie`](crate::Request::cookie).  As per [section 5.3 step 11 of RFC 6265], a new
    /// cookie is only treated as the same as an old one if it has a matching name, domain and
    /// path.
    ///
    /// The domain and path are not sent to the server on subsequent HTTP requests, so if a cookie
    /// was originally set with a domain and/or path, calling this function on a cookie with the
    /// same name but with either a different, or no, domain and/or path will lead to us sending an
    /// empty cookie that the user agent will treat as unrelated to the original one, and will thus
    /// not remove the old one.
    ///
    /// To avoid this you can manually set the [domain](Cookie::set_domain) and
    /// [path](Cookie::set_path) as necessary after retrieving the cookie using
    /// [`Request::cookie`](crate::Request::cookie).
    ///
    /// [section 5.3 step 11 of RFC 6265]: https://tools.ietf.org/html/rfc6265#section-5.3
    #[cfg(feature = "cookies")]
    pub fn remove_cookie(&mut self, cookie: Cookie<'static>) {
        panic!("STUB: not implemented");
    }
    /// Returns an optional reference to an error if the response contains one.
    pub fn error(&self) -> Option<&Error> {
        panic!("STUB: not implemented");
    }
    /// Returns a reference to the original error associated with this response if there is one and
    /// if it can be downcast to the specified type.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::io::ErrorKind;
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// use tide::Response;
    ///
    /// let error = std::io::Error::new(ErrorKind::Other, "oh no!");
    /// let error = tide::http::Error::from(error);
    ///
    /// let mut res = Response::new(400);
    /// res.set_error(error);
    ///
    /// if let Some(err) = res.downcast_error::<std::io::Error>() {
    ///   // Do something with the `std::io::Error`.
    /// }
    /// # Ok(())
    /// # })}
    pub fn downcast_error<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Takes the error from the response if one exists, replacing it with `None`.
    pub fn take_error(&mut self) -> Option<Error> {
        panic!("STUB: not implemented");
    }
    /// Sets the response's error, overwriting any existing error.
    ///
    /// This is particularly useful for middleware which would like to notify further
    /// middleware that an error has occurred without overwriting the existing response.
    pub fn set_error(&mut self, error: impl Into<Error>) {
        panic!("STUB: not implemented");
    }
    /// Get a response scoped extension value.
    #[must_use]
    pub fn ext<T: Send + Sync + 'static>(&self) -> Option<&T> {
        panic!("STUB: not implemented");
    }
    /// Set a response scoped extension value.
    pub fn insert_ext<T: Send + Sync + 'static>(&mut self, val: T) {
        panic!("STUB: not implemented");
    }
    /// Create a `tide::Response` from a type that can be converted into an
    /// `http_types::Response`.
    pub fn from_res<T>(value: T) -> Self
    where
        T: Into<http_types::Response>,
    {
        panic!("STUB: not implemented");
    }
}
impl AsRef<http::Response> for Response {
    fn as_ref(&self) -> &http::Response {
        panic!("STUB: not implemented");
    }
}
impl AsMut<http::Response> for Response {
    fn as_mut(&mut self) -> &mut http::Response {
        panic!("STUB: not implemented");
    }
}
impl AsRef<http::Headers> for Response {
    fn as_ref(&self) -> &http::Headers {
        panic!("STUB: not implemented");
    }
}
impl AsMut<http::Headers> for Response {
    fn as_mut(&mut self) -> &mut http::Headers {
        panic!("STUB: not implemented");
    }
}
impl From<Response> for http::Response {
    fn from(response: Response) -> http_types::Response {
        panic!("STUB: not implemented");
    }
}
impl From<http::Body> for Response {
    fn from(body: http::Body) -> Self {
        panic!("STUB: not implemented");
    }
}
impl From<serde_json::Value> for Response {
    fn from(json_value: serde_json::Value) -> Self {
        panic!("STUB: not implemented");
    }
}
impl From<Error> for Response {
    fn from(err: Error) -> Self {
        panic!("STUB: not implemented");
    }
}
impl From<http::Response> for Response {
    fn from(res: http::Response) -> Self {
        panic!("STUB: not implemented");
    }
}
impl From<StatusCode> for Response {
    fn from(status: StatusCode) -> Self {
        panic!("STUB: not implemented");
    }
}
impl From<String> for Response {
    fn from(s: String) -> Self {
        panic!("STUB: not implemented");
    }
}
impl<'a> From<&'a str> for Response {
    fn from(s: &'a str) -> Self {
        panic!("STUB: not implemented");
    }
}
impl IntoIterator for Response {
    type Item = (HeaderName, HeaderValues);
    type IntoIter = http_types::headers::IntoIter;
    /// Returns a iterator of references over the remaining items.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        panic!("STUB: not implemented");
    }
}
impl<'a> IntoIterator for &'a Response {
    type Item = (&'a HeaderName, &'a HeaderValues);
    type IntoIter = http_types::headers::Iter<'a>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        panic!("STUB: not implemented");
    }
}
impl<'a> IntoIterator for &'a mut Response {
    type Item = (&'a HeaderName, &'a mut HeaderValues);
    type IntoIter = http_types::headers::IterMut<'a>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        panic!("STUB: not implemented");
    }
}
impl Index<HeaderName> for Response {
    type Output = HeaderValues;
    /// Returns a reference to the value corresponding to the supplied name.
    ///
    /// # Panics
    ///
    /// Panics if the name is not present in `Response`.
    #[inline]
    fn index(&self, name: HeaderName) -> &HeaderValues {
        panic!("STUB: not implemented");
    }
}
impl Index<&str> for Response {
    type Output = HeaderValues;
    /// Returns a reference to the value corresponding to the supplied name.
    ///
    /// # Panics
    ///
    /// Panics if the name is not present in `Response`.
    #[inline]
    fn index(&self, name: &str) -> &HeaderValues {
        panic!("STUB: not implemented");
    }
}
