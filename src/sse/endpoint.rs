use crate::http::{mime, Body, StatusCode};
use crate::sse::Sender;
use crate::{Endpoint, Request, Response, Result};
use async_std::future::Future;
use async_std::io::BufReader;
use async_std::task;
use kv_log_macro::error;
use std::marker::PhantomData;
use std::sync::Arc;
/// Create an endpoint that can handle SSE connections.
pub fn endpoint<F, Fut, State>(handler: F) -> SseEndpoint<F, Fut, State>
where
    State: Clone + Send + Sync + 'static,
    F: Fn(Request<State>, Sender) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    panic!("STUB: not implemented");
}
/// An endpoint that can handle SSE connections.
#[derive(Debug)]
pub struct SseEndpoint<F, Fut, State>
where
    State: Clone + Send + Sync + 'static,
    F: Fn(Request<State>, Sender) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    handler: Arc<F>,
    __state: PhantomData<State>,
}
#[async_trait::async_trait]
impl<F, Fut, State> Endpoint<State> for SseEndpoint<F, Fut, State>
where
    State: Clone + Send + Sync + 'static,
    F: Fn(Request<State>, Sender) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    async fn call(&self, req: Request<State>) -> Result<Response> {
        panic!("STUB: not implemented");
    }
}
