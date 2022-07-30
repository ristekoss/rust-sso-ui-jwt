//! JWT utilities and claims for user data.

pub mod handler;
pub mod payload;

pub use handler::{create_token, decode_token};
pub use payload::{SSOJWTClaims, SSOUser, TokenType};
