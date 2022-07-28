use serde::{Deserialize, Serialize};

use crate::orgs::Organization;

#[derive(Deserialize, Serialize)]
pub struct SSOJWTClaims {
    pub iat: i64,
    pub exp: i64,

    pub nama: String,
    pub username: String,
    pub npm: String,
    pub organization: Organization,
}

#[derive(Serialize)]
pub enum TokenType {
    #[serde(rename = "access_token")]
    AccessToken,

    #[serde(rename = "refresh_token")]
    RefreshToken,
}
