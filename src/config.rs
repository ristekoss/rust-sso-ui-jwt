//! SSO and JWT configurations.

/// Struct to store your project configuration for SSO UI and JWT.
///
/// # Examples
///
/// ```rust
/// use sso_ui_jwt::SSOJWTConfig;
///
/// let config = SSOJWTConfig::new(
///     120,
///     120,
///     String::from("access secret"),
///     String::from("refresh secret"),
///     String::from("http://localhost:7700/login"),
///     String::from("http://localhost:7700"),
/// );
/// ```
#[derive(Clone, Debug)]
pub struct SSOJWTConfig {
    /// Expiration time for your access token (in seconds).
    pub access_token_exp_time: i64,
    /// Expiration time for your refresh token (in seconds).
    pub refresh_token_exp_time: i64,
    /// JWT secret for your access token.
    pub access_token_secret_key: String,
    /// JWT secret for your refresh token.
    pub refresh_token_secret_key: String,
    /// The service URL you used to sign in through SSO.
    pub service_url: String,
    /// The origin URL of your project.
    pub origin_url: String,
    /// SSO UI's CAS URL.
    pub cas_url: String,
}

impl SSOJWTConfig {
    /// Creates a new [`SSOJWTConfig`] instance.
    pub fn new(
        access_token_exp_time: i64,
        refresh_token_exp_time: i64,
        access_token_secret_key: String,
        refresh_token_secret_key: String,
        service_url: String,
        origin_url: String,
    ) -> Self {
        Self {
            access_token_exp_time,
            refresh_token_exp_time,
            access_token_secret_key,
            refresh_token_secret_key,
            service_url,
            origin_url,
            cas_url: String::from("https://sso.ui.ac.id/cas2"),
        }
    }
}
