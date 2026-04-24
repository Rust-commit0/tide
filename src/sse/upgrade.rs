use crate::http::{mime, Body, StatusCode};
use crate::{Request, Response, Result};
use super::Sender;
use async_std::future::Future;
use async_std::io::BufReader;
use async_std::task;
use kv_log_macro::error;
/// Upgrade an existing HTTP connection to an SSE connection.
pub fn upgrade<F, Fut, State>(req: Request<State>, handler: F) -> Response
where
    State: Clone + Send + Sync + 'static,
    F: Fn(Request<State>, Sender) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    panic!("STUB: not implemented");
}
