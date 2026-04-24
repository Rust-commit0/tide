use super::{Session, SessionStore};
use crate::http::{
    cookies::{Cookie, Key, SameSite},
    format_err,
};
use crate::{utils::async_trait, Middleware, Next, Request};
use std::time::Duration;
use async_session::{
    base64, hmac::{Hmac, Mac, NewMac},
    sha2::Sha256,
};
use kv_log_macro::error;
const BASE64_DIGEST_LEN: usize = 44;
/// # Middleware to enable sessions.
/// See [sessions](crate::sessions) for an overview of tide's approach to sessions.
///
/// ## Example
/// ```rust
/// # async_std::task::block_on(async {
/// let mut app = tide::new();
///
/// app.with(tide::sessions::SessionMiddleware::new(
///     tide::sessions::MemoryStore::new(),
///     b"we recommend you use std::env::var(\"TIDE_SECRET\").unwrap().as_bytes() instead of a fixed value"
/// ));
///
/// app.with(tide::utils::Before(|mut request: tide::Request<()>| async move {
///     let session = request.session_mut();
///     let visits: usize = session.get("visits").unwrap_or_default();
///     session.insert("visits", visits + 1).unwrap();
///     request
/// }));
///
/// app.at("/").get(|req: tide::Request<()>| async move {
///     let visits: usize = req.session().get("visits").unwrap();
///     Ok(format!("you have visited this website {} times", visits))
/// });
///
/// app.at("/reset")
///     .get(|mut req: tide::Request<()>| async move {
///         req.session_mut().destroy();
///         Ok(tide::Redirect::new("/"))
///      });
/// # })
/// ```
pub struct SessionMiddleware<Store> {
    store: Store,
    cookie_path: String,
    cookie_name: String,
    cookie_domain: Option<String>,
    session_ttl: Option<Duration>,
    save_unchanged: bool,
    secure: Option<bool>,
    same_site_policy: SameSite,
    key: Key,
}
impl<Store: SessionStore> std::fmt::Debug for SessionMiddleware<Store> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("STUB: not implemented");
    }
}
#[async_trait]
impl<Store, State> Middleware<State> for SessionMiddleware<Store>
where
    Store: SessionStore,
    State: Clone + Send + Sync + 'static,
{
    async fn handle(
        &self,
        mut request: Request<State>,
        next: Next<'_, State>,
    ) -> crate::Result {
        panic!("STUB: not implemented");
    }
}
impl<Store: SessionStore> SessionMiddleware<Store> {
    /// Creates a new SessionMiddleware with a mandatory cookie
    /// signing secret. The `secret` MUST be at least 32 bytes long,
    /// and MUST be cryptographically random to be secure. It is
    /// recommended to retrieve this at runtime from the environment
    /// instead of compiling it into your
    /// application.
    ///
    /// # Panics
    ///
    /// SessionMiddleware::new will panic if the secret is fewer than
    /// 32 bytes.
    ///
    /// # Defaults
    ///
    /// The defaults for SessionMiddleware are:
    /// * cookie path: "/"
    /// * cookie name: "tide.sid"
    /// * session ttl: one day
    /// * secure: request.scheme == 'https'
    /// * same site: strict
    /// * save unchanged: enabled
    ///
    /// # Customization
    ///
    /// Although the above defaults are appropriate for most
    /// applications, they can be overridden. Please be careful
    /// changing these settings, as they can weaken your application's
    /// security:
    ///
    /// ```rust
    /// # use tide::http::cookies::SameSite;
    /// # use std::time::Duration;
    /// # use tide::sessions::{SessionMiddleware, MemoryStore};
    /// let mut app = tide::new();
    /// app.with(
    ///     SessionMiddleware::new(MemoryStore::new(), b"please do not hardcode your secret")
    ///         .with_cookie_name("custom.cookie.name")
    ///         .with_cookie_path("/some/path")
    ///         .with_cookie_domain("www.rust-lang.org")
    ///         .with_secure(true)
    ///         .with_same_site_policy(SameSite::Lax)
    ///         .with_session_ttl(Some(Duration::from_secs(1)))
    ///         .without_save_unchanged(),
    /// );
    /// ```
    pub fn new(store: Store, secret: &[u8]) -> Self {
        panic!("STUB: not implemented");
    }
    /// Sets a cookie path for this session middleware.
    /// The default for this value is "/"
    pub fn with_cookie_path(mut self, cookie_path: impl AsRef<str>) -> Self {
        panic!("STUB: not implemented");
    }
    /// Sets a session ttl. This will be used both for the cookie
    /// expiry and also for the session-internal expiry.
    ///
    /// The default for this value is one day. Set this to None to not
    /// set a cookie or session expiry. This is not recommended.
    pub fn with_session_ttl(mut self, session_ttl: Option<Duration>) -> Self {
        panic!("STUB: not implemented");
    }
    /// Sets the name of the cookie that the session is stored with or in.
    ///
    /// If you are running multiple tide applications on the same
    /// domain, you will need different values for each
    /// application. The default value is "tide.sid"
    pub fn with_cookie_name(mut self, cookie_name: impl AsRef<str>) -> Self {
        panic!("STUB: not implemented");
    }
    /// Disables the `save_unchanged` setting. When `save_unchanged`
    /// is enabled, a session will cookie will always be set. With
    /// `save_unchanged` disabled, the session data must be modified
    /// from the `Default` value in order for it to save. If a session
    /// already exists and its data unmodified in the course of a
    /// request, the session will only be persisted if
    /// `save_unchanged` is enabled.
    pub fn without_save_unchanged(mut self) -> Self {
        panic!("STUB: not implemented");
    }
    /// Sets the secure attribute of the cookie.
    /// Defaults to true if the incoming request scheme is 'https'
    /// Can optionally be set to true or false to override
    pub fn with_secure(mut self, secure: bool) -> Self {
        panic!("STUB: not implemented");
    }
    /// Sets the same site policy for the session cookie. Defaults to
    /// SameSite::Lax. See [incrementally better
    /// cookies](https://tools.ietf.org/html/draft-west-cookie-incrementalism-01)
    /// for more information about this setting
    pub fn with_same_site_policy(mut self, policy: SameSite) -> Self {
        panic!("STUB: not implemented");
    }
    /// Sets the domain of the cookie.
    pub fn with_cookie_domain(mut self, cookie_domain: impl AsRef<str>) -> Self {
        panic!("STUB: not implemented");
    }
    async fn load_or_create(&self, cookie_value: Option<String>) -> Session {
        panic!("STUB: not implemented");
    }
    fn build_cookie(&self, secure: bool, cookie_value: String) -> Cookie<'static> {
        panic!("STUB: not implemented");
    }
    /// Signs the cookie's value providing integrity and authenticity.
    fn sign_cookie(&self, cookie: &mut Cookie<'_>) {
        panic!("STUB: not implemented");
    }
    /// Given a signed value `str` where the signature is prepended to `value`,
    /// verifies the signed value and returns it. If there's a problem, returns
    /// an `Err` with a string describing the issue.
    fn verify_signature(&self, cookie_value: &str) -> Result<String, &'static str> {
        panic!("STUB: not implemented");
    }
}
