//! Ticket validation handlers.

use reqwest::Client;
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
/// - [`BadRequest`][validate_ticket_error]: Bad request to CAS server
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
/// use tokio;
///
/// #[tokio::main]
/// async fn main() {
///     let config = SSOJWTConfig::new(
///         120,
///         120,
///         String::from("access secret"),
///         String::from("refresh secret"),
///         String::from("http://some-service/login"),
///         String::from("http://some-service"),
///     );
///
///     let response = validate_ticket(&config, "a ticket").await;
///     let status = if let Err(ValidateTicketError::AuthenticationFailed) = response {
///         "failed"
///     } else {
///         "success"
///     };
///
///     assert_eq!(status, "failed");
/// }
/// ```
pub async fn validate_ticket(
    config: &SSOJWTConfig,
    ticket: &str,
) -> Result<ServiceResponse, ValidateTicketError> {
    let client = Client::new();
    let url = format!(
        "{}/serviceValidate?ticket={}&service={}",
        config.cas_url, ticket, config.service_url
    );

    let response = match client
        .get(&url)
        .header("Host", "sso.ui.ac.id")
        .header("User-Agent", "Node-Fetch")
        .send()
        .await
    {
        Ok(res) => res,
        Err(_err) => {
            #[cfg(feature = "log")]
            log::error!("{}", _err);

            return Err(ValidateTicketError::ReqwestError);
        }
    };

    let content = match response.text().await {
        Ok(content) => content,
        Err(_err) => {
            #[cfg(feature = "log")]
            log::error!("{}", _err);

            return Err(ValidateTicketError::XMLParsingError);
        }
    };

    let response = ServiceResponse::from_str(&content).unwrap();

    match response.authentication_success {
        Some(_) => Ok(response),
        None => Err(ValidateTicketError::AuthenticationFailed),
    }
}
