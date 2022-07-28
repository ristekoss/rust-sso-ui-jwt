#[cfg(feature = "log")]
use crate::log::init_logger;

#[derive(Clone)]
pub struct SSOJWTConfig {
    pub access_token_exp_time: i64,
    pub refresh_token_exp_time: i64,
    pub access_token_secret_key: String,
    pub refresh_token_secret_key: String,
    pub service_url: String,
    pub origin_url: String,
    pub cas_url: String,
}

impl SSOJWTConfig {
    pub fn new(
        access_token_exp_time: i64,
        refresh_token_exp_time: i64,
        access_token_secret_key: String,
        refresh_token_secret_key: String,
        service_url: String,
        origin_url: String,
    ) -> Self {
        #[cfg(feature = "log")]
        init_logger();

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
