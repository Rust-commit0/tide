use routefinder::{Captures, Router as MethodRouter};
use std::collections::HashMap;
use crate::endpoint::DynEndpoint;
use crate::{Request, Response, StatusCode};
/// The routing table used by `Server`
///
/// Internally, we have a separate state machine per http method; indexing
/// by the method first allows the table itself to be more efficient.
#[allow(missing_debug_implementations)]
pub(crate) struct Router<State> {
    method_map: HashMap<http_types::Method, MethodRouter<Box<DynEndpoint<State>>>>,
    all_method_router: MethodRouter<Box<DynEndpoint<State>>>,
}
impl<State> std::fmt::Debug for Router<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("STUB: not implemented");
    }
}
/// The result of routing a URL
pub(crate) struct Selection<'a, State> {
    pub(crate) endpoint: &'a DynEndpoint<State>,
    pub(crate) params: Captures<'static, 'static>,
}
impl<State: Clone + Send + Sync + 'static> Router<State> {
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn add(
        &mut self,
        path: &str,
        method: http_types::Method,
        ep: Box<DynEndpoint<State>>,
    ) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn add_all(&mut self, path: &str, ep: Box<DynEndpoint<State>>) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn route(
        &self,
        path: &str,
        method: http_types::Method,
    ) -> Selection<'_, State> {
        panic!("STUB: not implemented");
    }
}
async fn not_found_endpoint<State: Clone + Send + Sync + 'static>(
    _req: Request<State>,
) -> crate::Result {
    panic!("STUB: not implemented");
}
async fn method_not_allowed<State: Clone + Send + Sync + 'static>(
    _req: Request<State>,
) -> crate::Result {
    panic!("STUB: not implemented");
}
