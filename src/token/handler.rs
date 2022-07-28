use jsonwebtoken::{
    decode, encode, errors::Result, DecodingKey, EncodingKey, Header, TokenData, Validation,
};

use crate::{orgs::get_organizations, ticket::ServiceResponse, SSOJWTConfig};

use super::{payload::SSOJWTClaims, TokenType};

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
