use http_types::headers::{HeaderValue, HeaderValues};
use http_types::{headers, Method, StatusCode};
use regex::Regex;
use std::hash::Hash;
use crate::middleware::{Middleware, Next};
use crate::{Request, Result};
/// Middleware for CORS
///
/// # Example
///
/// ```no_run
/// use http_types::headers::HeaderValue;
/// use tide::security::{CorsMiddleware, Origin};
///
/// let cors = CorsMiddleware::new()
///     .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
///     .allow_origin(Origin::from("*"))
///     .allow_credentials(false);
/// ```
#[derive(Clone, Debug, Hash)]
pub struct CorsMiddleware {
    allow_credentials: Option<HeaderValue>,
    allow_headers: HeaderValue,
    allow_methods: HeaderValue,
    allow_origin: Origin,
    expose_headers: Option<HeaderValue>,
    max_age: HeaderValue,
}
pub(crate) const DEFAULT_MAX_AGE: &str = "86400";
pub(crate) const DEFAULT_METHODS: &str = "GET, POST, OPTIONS";
pub(crate) const WILDCARD: &str = "*";
impl CorsMiddleware {
    /// Creates a new Cors middleware.
    #[must_use]
    pub fn new() -> Self {
        panic!("STUB: not implemented");
    }
    /// Set `allow_credentials` and return new Cors
    #[must_use]
    pub fn allow_credentials(mut self, allow_credentials: bool) -> Self {
        panic!("STUB: not implemented");
    }
    /// Set `allow_headers` and return new Cors
    pub fn allow_headers<T: Into<HeaderValue>>(mut self, headers: T) -> Self {
        panic!("STUB: not implemented");
    }
    /// Set `max_age` and return new Cors
    pub fn max_age<T: Into<HeaderValue>>(mut self, max_age: T) -> Self {
        panic!("STUB: not implemented");
    }
    /// Set `allow_methods` and return new Cors
    pub fn allow_methods<T: Into<HeaderValue>>(mut self, methods: T) -> Self {
        panic!("STUB: not implemented");
    }
    /// Set `allow_origin` and return new Cors
    pub fn allow_origin<T: Into<Origin>>(mut self, origin: T) -> Self {
        panic!("STUB: not implemented");
    }
    /// Set `expose_headers` and return new Cors
    pub fn expose_headers<T: Into<HeaderValue>>(mut self, headers: T) -> Self {
        panic!("STUB: not implemented");
    }
    fn build_preflight_response(&self, origins: &HeaderValues) -> http_types::Response {
        panic!("STUB: not implemented");
    }
    /// Look at origin of request and determine `allow_origin`
    fn response_origin(&self, origin: &HeaderValue) -> HeaderValue {
        panic!("STUB: not implemented");
    }
    /// Determine if origin is appropriate
    fn is_valid_origin(&self, origin: &HeaderValue) -> bool {
        panic!("STUB: not implemented");
    }
}
#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for CorsMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        panic!("STUB: not implemented");
    }
}
impl Default for CorsMiddleware {
    fn default() -> Self {
        panic!("STUB: not implemented");
    }
}
/// `allow_origin` enum
#[derive(Clone, Debug)]
pub enum Origin {
    /// Wildcard. Accept all origin requests
    Any,
    /// Set a single allow_origin target
    Exact(String),
    /// Set multiple allow_origin targets
    List(Vec<String>),
    /// Set a regex allow_origin targets
    Match(Regex),
}
impl From<String> for Origin {
    fn from(s: String) -> Self {
        panic!("STUB: not implemented");
    }
}
impl From<&str> for Origin {
    fn from(s: &str) -> Self {
        panic!("STUB: not implemented");
    }
}
impl From<Vec<String>> for Origin {
    fn from(list: Vec<String>) -> Self {
        panic!("STUB: not implemented");
    }
}
impl From<Regex> for Origin {
    fn from(regex: Regex) -> Self {
        panic!("STUB: not implemented");
    }
}
impl From<Vec<&str>> for Origin {
    fn from(list: Vec<&str>) -> Self {
        panic!("STUB: not implemented");
    }
}
impl PartialEq for Origin {
    fn eq(&self, other: &Self) -> bool {
        panic!("STUB: not implemented");
    }
}
impl Hash for Origin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use http_types::headers::{self, HeaderValue};
    const ALLOW_ORIGIN: &str = "example.com";
    const ALLOW_METHODS: &str = "GET, POST, OPTIONS, DELETE";
    const EXPOSE_HEADER: &str = "X-My-Custom-Header";
    const ENDPOINT: &str = "/cors";
    fn endpoint_url() -> http_types::Url {
        format!("http://{}{}", ALLOW_ORIGIN, ENDPOINT).parse().unwrap()
    }
    fn app() -> crate::Server<()> {
        let mut app = crate::Server::new();
        app.at(ENDPOINT).get(|_| async { Ok("Hello World") });
        app
    }
    fn request() -> http_types::Request {
        let mut req = http_types::Request::new(http_types::Method::Get, endpoint_url());
        req.insert_header(http_types::headers::ORIGIN, ALLOW_ORIGIN);
        req
    }
    #[async_std::test]
    async fn preflight_request() {
        let mut app = app();
        app.with(
            CorsMiddleware::new()
                .allow_origin(Origin::from(ALLOW_ORIGIN))
                .allow_methods(ALLOW_METHODS.parse::<HeaderValue>().unwrap())
                .expose_headers(EXPOSE_HEADER.parse::<HeaderValue>().unwrap())
                .allow_credentials(true),
        );
        let mut req = http_types::Request::new(
            http_types::Method::Options,
            endpoint_url(),
        );
        req.insert_header(http_types::headers::ORIGIN, ALLOW_ORIGIN);
        let res: crate::http::Response = app.respond(req).await.unwrap();
        assert_eq!(res.status(), 200);
        assert_eq!(res[headers::ACCESS_CONTROL_ALLOW_ORIGIN], ALLOW_ORIGIN);
        assert_eq!(res[headers::ACCESS_CONTROL_ALLOW_METHODS], ALLOW_METHODS);
        assert_eq!(res[headers::ACCESS_CONTROL_ALLOW_HEADERS], WILDCARD);
        assert_eq!(res[headers::ACCESS_CONTROL_MAX_AGE], DEFAULT_MAX_AGE);
        assert_eq!(res[headers::ACCESS_CONTROL_ALLOW_CREDENTIALS], "true");
    }
    #[async_std::test]
    async fn default_cors_middleware() {
        let mut app = app();
        app.with(CorsMiddleware::new());
        let res: crate::http::Response = app.respond(request()).await.unwrap();
        assert_eq!(res.status(), 200);
        assert_eq!(res[headers::ACCESS_CONTROL_ALLOW_ORIGIN], "*");
    }
    #[async_std::test]
    async fn custom_cors_middleware() {
        let mut app = app();
        app.with(
            CorsMiddleware::new()
                .allow_origin(Origin::from(ALLOW_ORIGIN))
                .allow_credentials(false)
                .allow_methods(ALLOW_METHODS.parse::<HeaderValue>().unwrap())
                .expose_headers(EXPOSE_HEADER.parse::<HeaderValue>().unwrap()),
        );
        let res: crate::http::Response = app.respond(request()).await.unwrap();
        assert_eq!(res.status(), 200);
        assert_eq!(res[headers::ACCESS_CONTROL_ALLOW_ORIGIN], ALLOW_ORIGIN);
    }
    #[async_std::test]
    async fn regex_cors_middleware() {
        let regex = Regex::new(r"e[xzs]a.*le.com*").unwrap();
        let mut app = app();
        app.with(
            CorsMiddleware::new()
                .allow_origin(Origin::from(regex))
                .allow_credentials(false)
                .allow_methods(ALLOW_METHODS.parse::<HeaderValue>().unwrap())
                .expose_headers(EXPOSE_HEADER.parse::<HeaderValue>().unwrap()),
        );
        let res: crate::http::Response = app.respond(request()).await.unwrap();
        assert_eq!(res.status(), 200);
        assert_eq!(res[headers::ACCESS_CONTROL_ALLOW_ORIGIN], ALLOW_ORIGIN);
    }
    #[async_std::test]
    async fn credentials_true() {
        let mut app = app();
        app.with(CorsMiddleware::new().allow_credentials(true));
        let res: crate::http::Response = app.respond(request()).await.unwrap();
        assert_eq!(res.status(), 200);
        assert_eq!(res[headers::ACCESS_CONTROL_ALLOW_CREDENTIALS], "true");
    }
    #[async_std::test]
    async fn set_allow_origin_list() {
        let mut app = app();
        let origins = vec![ALLOW_ORIGIN, "foo.com", "bar.com"];
        app.with(CorsMiddleware::new().allow_origin(origins.clone()));
        for origin in origins {
            let mut req = http_types::Request::new(
                http_types::Method::Get,
                endpoint_url(),
            );
            req.insert_header(http_types::headers::ORIGIN, origin);
            let res: crate::http::Response = app.respond(req).await.unwrap();
            assert_eq!(res.status(), 200);
            assert_eq!(res[headers::ACCESS_CONTROL_ALLOW_ORIGIN] [0], origin);
        }
    }
    #[async_std::test]
    async fn not_set_origin_header() {
        let mut app = app();
        app.with(CorsMiddleware::new().allow_origin(ALLOW_ORIGIN));
        let req = crate::http::Request::new(http_types::Method::Get, endpoint_url());
        let res: crate::http::Response = app.respond(req).await.unwrap();
        assert_eq!(res.status(), 200);
    }
    #[async_std::test]
    async fn unauthorized_origin() {
        let mut app = app();
        app.with(CorsMiddleware::new().allow_origin(ALLOW_ORIGIN));
        let mut req = http_types::Request::new(http_types::Method::Get, endpoint_url());
        req.insert_header(http_types::headers::ORIGIN, "unauthorize-origin.net");
        let res: crate::http::Response = app.respond(req).await.unwrap();
        assert_eq!(res.status(), 401);
    }
    #[async_std::test]
    #[cfg(feature = "cookies")]
    async fn retain_cookies() {
        let mut app = crate::Server::new();
        app.with(CorsMiddleware::new().allow_origin(ALLOW_ORIGIN));
        app.at(ENDPOINT)
            .get(|_| async {
                let mut res = crate::Response::new(http_types::StatusCode::Ok);
                res.insert_cookie(http_types::Cookie::new("foo", "bar"));
                Ok(res)
            });
        let mut req = http_types::Request::new(http_types::Method::Get, endpoint_url());
        req.insert_header(http_types::headers::ORIGIN, ALLOW_ORIGIN);
        let res: crate::http::Response = app.respond(req).await.unwrap();
        assert_eq!(res[http_types::headers::SET_COOKIE] [0], "foo=bar");
    }
    #[async_std::test]
    async fn set_cors_headers_to_error_responses() {
        let mut app = crate::Server::new();
        app.at(ENDPOINT)
            .get(|_| async {
                Err::<
                    &str,
                    _,
                >(crate::Error::from_str(StatusCode::BadRequest, "bad request"))
            });
        app.with(CorsMiddleware::new().allow_origin(Origin::from(ALLOW_ORIGIN)));
        let res: crate::http::Response = app.respond(request()).await.unwrap();
        assert_eq!(res.status(), 400);
        assert_eq!(res[headers::ACCESS_CONTROL_ALLOW_ORIGIN], ALLOW_ORIGIN);
    }
    #[cfg(test)]
    mod origin {
        use super::super::Origin;
        use regex::Regex;
        #[test]
        fn transitive() {
            let regex = Regex::new(r"e[xzs]a.*le.com*").unwrap();
            let x = Origin::from(regex.clone());
            let y = Origin::from(regex.clone());
            let z = Origin::from(regex);
            assert!(x == y && y == z && x == z);
        }
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn symetrical() {
            let regex = Regex::new(r"e[xzs]a.*le.com*").unwrap();
            let x = Origin::from(regex.clone());
            let y = Origin::from(regex);
            assert!(x == y && y == x);
        }
        #[test]
        fn reflexive() {
            let regex = Regex::new(r"e[xzs]a.*le.com*").unwrap();
            let x = Origin::from(regex);
            assert!(x == x);
        }
    }
}
