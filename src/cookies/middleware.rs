use crate::response::CookieEvent;
use crate::{Middleware, Next, Request};
use async_trait::async_trait;
use crate::http::cookies::{Cookie, CookieJar, Delta};
use crate::http::headers;
use std::sync::{Arc, RwLock};
/// A middleware for making cookie data available in requests.
///
/// # Examples
///
/// ```
/// # use tide::{Request, Response, StatusCode};
/// # use tide::http::cookies::Cookie;
/// let mut app = tide::Server::new();
/// app.at("/get").get(|req: Request<()>| async move {
///     Ok(req.cookie("testCookie").unwrap().value().to_string())
/// });
/// app.at("/set").get(|_| async {
///     let mut res = Response::new(StatusCode::Ok);
///     res.insert_cookie(Cookie::new("testCookie", "NewCookieValue"));
///     Ok(res)
/// });
/// ```
#[derive(Debug, Clone, Default)]
pub(crate) struct CookiesMiddleware;
impl CookiesMiddleware {
    /// Creates a new `CookiesMiddleware`.
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
}
#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for CookiesMiddleware {
    async fn handle(
        &self,
        mut ctx: Request<State>,
        next: Next<'_, State>,
    ) -> crate::Result {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Default, Clone)]
pub(crate) struct CookieData {
    pub(crate) content: Arc<RwLock<LazyJar>>,
}
#[derive(Debug, Default, Clone)]
/// Wrapper around `CookieJar`, that initializes only when actually used.
pub(crate) struct LazyJar(Option<CookieJar>);
impl LazyJar {
    fn add(&mut self, cookie: Cookie<'static>) {
        panic!("STUB: not implemented");
    }
    fn remove(&mut self, cookie: Cookie<'static>) {
        panic!("STUB: not implemented");
    }
    fn delta(&mut self) -> Delta<'_> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn get(&self, name: &str) -> Option<&Cookie<'static>> {
        panic!("STUB: not implemented");
    }
    fn get_jar(&mut self) -> &mut CookieJar {
        panic!("STUB: not implemented");
    }
}
impl CookieData {
    pub(crate) fn from_request<S>(req: &Request<S>) -> Self {
        panic!("STUB: not implemented");
    }
}
