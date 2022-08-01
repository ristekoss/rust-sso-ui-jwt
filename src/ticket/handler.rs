//! Ticket validation handlers.

use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use strong_xml::XmlRead;

use crate::SSOJWTConfig;

use super::{ServiceResponse, ValidateTicketError};

/// Validates a ticket returned from the CAS SSO server.
///
/// # Errors
///
/// - [`AuthenticationFailed`][validate_ticket_error]: Failed ticket authentication
/// - [`ReqwestError`][validate_ticket_error]: Validation request caused an error
/// - [`XMLParsingError`][validate_ticket_error]: Error parsing XML response
///
/// [validate_ticket_error]: crate::ticket::error::ValidateTicketError
///
/// # Examples
///
/// ```rust
/// use sso_ui_jwt::{
///     ticket::{validate_ticket, ValidateTicketError},
///     SSOJWTConfig,
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
/// let response = validate_ticket(&config, "a ticket");
/// let status = if let Err(ValidateTicketError::XMLParsingError) = response {
///     "failed"
/// } else {
///     "success"
/// };
///
/// assert_eq!(status, "failed");
/// ```
pub fn validate_ticket(
    config: &SSOJWTConfig,
    ticket: &str,
) -> Result<ServiceResponse, ValidateTicketError> {
    let mut writer = Vec::new();
    let url = format!(
        "{}/serviceValidate?ticket={}&service={}",
        config.cas_url, ticket, config.service_url
    );
    let uri = Uri::try_from(url.as_str()).unwrap();
    let mut request = Request::new(&uri);

    let request = request
        .method(Method::GET)
        .header("Host", "sso.ui.ac.id")
        .header("User-Agent", "Node-Fetch");

    match request.send(&mut writer) {
        Ok(res) => res,
        Err(_err) => {
            #[cfg(feature = "log")]
            log::error!("{}", _err);

            return Err(ValidateTicketError::ReqwestError);
        }
    };

    let content = String::from_utf8_lossy(&writer);

    let response = match ServiceResponse::from_str(&content) {
        Ok(response) => response,
        Err(_err) => {
            #[cfg(feature = "log")]
            log::error!("{}", _err);

            return Err(ValidateTicketError::XMLParsingError);
        }
    };

    match response.authentication_success {
        Some(_) => Ok(response),
        None => Err(ValidateTicketError::AuthenticationFailed),
    }
}
