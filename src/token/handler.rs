//! JWT creation and decoding handlers.

use jsonwebtoken::{
    decode, encode, errors::Result, DecodingKey, EncodingKey, Header, TokenData, Validation,
};

use crate::{orgs::get_organizations, ticket::ServiceResponse, SSOJWTConfig};

use super::{payload::SSOJWTClaims, TokenType};

/// Creates a JWT from service response.
///
/// # Errors
///
/// - [`jsonwebtoken::Error`][jwterror]: Errors from the [`jsonwebtoken`] crate.
///
/// [jwterror]: jsonwebtoken::errors::Error
/// [`jsonwebtoken`]: jsonwebtoken
///
/// # Examples
///
/// ```rust
/// use sso_ui_jwt::{
///     ticket::{Attributes, AuthenticationSuccess, ServiceResponse},
///     token::{create_token, TokenType},
///     SSOJWTConfig
/// };
///
/// let config = SSOJWTConfig::new(
///     120,
///     120,
///     String::from("access secret"),
///     String::from("refresh secret"),
///     String::from("http://some-service/login"),
///     String::from("http://some-service"),
/// );
///
/// let service_res = ServiceResponse {
///     authentication_success: Some(
///         AuthenticationSuccess {
///             user: String::from("username"),
///             attributes: Attributes {
///                 ldap_cn: String::from("Username"),
///                 kd_org: String::from("01.00.12.01"),
///                 peran_user: String::from("mahasiswa"),
///                 nama: String::from("Nama"),
///                 npm: String::from("Username"),
///             }
///         }
///     ),
/// };
///
/// let result = create_token(&config, TokenType::AccessToken, service_res).unwrap();
/// ```
pub fn create_token(
    config: &SSOJWTConfig,
    r#type: TokenType,
    service_res: ServiceResponse,
) -> Result<String> {
    let token_exp_time = match r#type {
        TokenType::AccessToken => config.access_token_exp_time,
        TokenType::RefreshToken => config.refresh_token_exp_time,
    };
    let token_secret_key = match r#type {
        TokenType::AccessToken => &config.access_token_secret_key,
        TokenType::RefreshToken => &config.refresh_token_secret_key,
    };

    let now = chrono::offset::Local::now();
    let exp = now.timestamp() + token_exp_time;

    let user_attributes = service_res.authentication_success.unwrap();

    let orgs = get_organizations();
    let org = orgs.get(&user_attributes.attributes.kd_org).unwrap();

    let claims = SSOJWTClaims {
        iat: now.timestamp(),
        exp,
        username: user_attributes.user,
        nama: user_attributes.attributes.nama,
        npm: user_attributes.attributes.npm,
        organization: org.clone(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(token_secret_key.as_bytes()),
    )
}

/// Decodes the JWT into its data
///
/// # Errors
///
/// Errors could occur when decoding the JWT. Those cases are:
///
/// - Passing a JWT with invalid claims.
/// - An expired JWT is passed to the function
/// - Decoding a JWT with the incorrect type
///
/// [jwterror]: jsonwebtoken::errors::Error
/// [`jsonwebtoken`]: jsonwebtoken
///
/// # Examples
///
/// ```rust
/// use sso_ui_jwt::{
///     ticket::{Attributes, AuthenticationSuccess, ServiceResponse},
///     token::{create_token, decode_token, TokenType},
///     SSOJWTConfig
/// };
///
/// let config = SSOJWTConfig::new(
///     120,
///     120,
///     String::from("access secret"),
///     String::from("refresh secret"),
///     String::from("http://some-service/login"),
///     String::from("http://some-service"),
/// );
///
/// let service_res = ServiceResponse {
///     authentication_success: Some(
///         AuthenticationSuccess {
///             user: String::from("username"),
///             attributes: Attributes {
///                 ldap_cn: String::from("Username"),
///                 kd_org: String::from("01.00.12.01"),
///                 peran_user: String::from("mahasiswa"),
///                 nama: String::from("Nama"),
///                 npm: String::from("Username"),
///             }
///         }
///     ),
/// };
///
/// let res = service_res.clone();
/// let token = create_token(&config, TokenType::RefreshToken, service_res).unwrap();
///
/// let claims = decode_token(&config, TokenType::RefreshToken, &token).unwrap().claims;
///
/// assert_eq!(
///     res.authentication_success.unwrap().user,
///     claims.username
/// );
/// ```
pub fn decode_token(
    config: &SSOJWTConfig,
    r#type: TokenType,
    token: &str,
) -> Result<TokenData<SSOJWTClaims>> {
    let token_secret_key = match r#type {
        TokenType::AccessToken => &config.access_token_secret_key,
        TokenType::RefreshToken => &config.refresh_token_secret_key,
    };

    decode::<SSOJWTClaims>(
        token,
        &DecodingKey::from_secret(token_secret_key.as_bytes()),
        &Validation::default(),
    )
}
