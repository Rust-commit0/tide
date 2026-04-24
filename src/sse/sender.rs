/// An SSE message sender.
#[derive(Debug)]
pub struct Sender {
    sender: async_sse::Sender,
}
impl Sender {
    /// Create a new instance of `Sender`.
    pub(crate) fn new(sender: async_sse::Sender) -> Self {
        panic!("STUB: not implemented");
    }
    /// Send data from the SSE channel.
    ///
    /// Each message consists of a "name" and "data".
    pub async fn send(
        &self,
        name: &str,
        data: impl AsRef<str>,
        id: Option<&str>,
    ) -> async_std::io::Result<()> {
        panic!("STUB: not implemented");
    }
}
