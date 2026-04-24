use kv_log_macro::{error, info, warn};
use crate::{Middleware, Next, Request};
/// Log all incoming requests and responses.
///
/// In the case of nested applications, this middleware will only run once for each request.
///
/// # Examples
///
/// ```
/// let mut app = tide::Server::new();
/// app.with(tide::log::LogMiddleware::new());
/// ```
#[derive(Debug, Default, Clone)]
pub struct LogMiddleware {
    _priv: (),
}
struct LogMiddlewareHasBeenRun;
impl LogMiddleware {
    /// Create a new instance of `LogMiddleware`.
    #[must_use]
    pub fn new() -> Self {
        panic!("STUB: not implemented");
    }
    /// Log a request and a response.
    async fn log<'a, State: Clone + Send + Sync + 'static>(
        &'a self,
        mut req: Request<State>,
        next: Next<'a, State>,
    ) -> crate::Result {
        panic!("STUB: not implemented");
    }
}
#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for LogMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> crate::Result {
        panic!("STUB: not implemented");
    }
}
