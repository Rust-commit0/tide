use crate::{Body, Endpoint, Request, Response, Result, StatusCode};
use async_std::path::PathBuf as AsyncPathBuf;
use kv_log_macro::{info, warn};
use std::path::{Path, PathBuf};
use std::{ffi::OsStr, io};
pub(crate) struct ServeDir {
    prefix: String,
    dir: PathBuf,
}
impl ServeDir {
    /// Create a new instance of `ServeDir`.
    pub(crate) fn new(prefix: String, dir: PathBuf) -> Self {
        panic!("STUB: not implemented");
    }
}
#[async_trait::async_trait]
impl<State> Endpoint<State> for ServeDir
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, req: Request<State>) -> Result {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    fn serve_dir(tempdir: &tempfile::TempDir) -> crate::Result<ServeDir> {
        let static_dir = tempdir.path().join("static");
        fs::create_dir(&static_dir)?;
        let file_path = static_dir.join("foo");
        let mut file = File::create(file_path)?;
        write!(file, "Foobar")?;
        Ok(ServeDir {
            prefix: "/static/".to_string(),
            dir: static_dir,
        })
    }
    fn request(path: &str) -> crate::Request<()> {
        let request = crate::http::Request::get(
            crate::http::Url::parse(&format!("http://localhost/{}", path)).unwrap(),
        );
        crate::Request::new((), request, vec![])
    }
    #[async_std::test]
    async fn ok() {
        let tempdir = tempfile::tempdir().unwrap();
        let serve_dir = serve_dir(&tempdir).unwrap();
        let req = request("static/foo");
        let res = serve_dir.call(req).await.unwrap();
        let mut res: crate::http::Response = res.into();
        assert_eq!(res.status(), 200);
        assert_eq!(res.body_string(). await .unwrap(), "Foobar");
    }
    #[async_std::test]
    async fn not_found() {
        let tempdir = tempfile::tempdir().unwrap();
        let serve_dir = serve_dir(&tempdir).unwrap();
        let req = request("static/bar");
        let res = serve_dir.call(req).await.unwrap();
        let res: crate::http::Response = res.into();
        assert_eq!(res.status(), 404);
    }
}
