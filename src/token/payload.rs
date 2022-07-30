//! JWT claims and token types.

use serde::{Deserialize, Serialize};

use crate::orgs::Organization;

/// JWT claims which contains user data.
#[derive(Deserialize, Serialize)]
pub struct SSOJWTClaims {
    /// Time JWT is issued at (in seconds).
    pub iat: i64,
    /// Time JWT expires (in seconds).
    pub exp: i64,

    /// User's name
    pub nama: String,
    /// User's username
    pub username: String,
    /// User's student id number
    pub npm: String,
    /// User's organization
    pub organization: Organization,
}

/// Types of JWT to create
#[derive(Serialize)]
pub enum TokenType {
    /// An access token for user authorization
    #[serde(rename = "access_token")]
    AccessToken,

    /// A refresh token to get new access token and generate new refresh token
    #[serde(rename = "refresh_token")]
    RefreshToken,
}
