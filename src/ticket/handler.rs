use reqwest::Client;
use strong_xml::XmlRead;

use crate::SSOJWTConfig;

use super::{ServiceResponse, ValidateTicketError};

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
        Err(err) => {
            #[cfg(feature = "log")]
            log::error!("{err}");

            return Err(ValidateTicketError::ReqwestError);
        }
    };

    let content = match response.text().await {
        Ok(content) => content,
        Err(err) => {
            #[cfg(feature = "log")]
            log::error!("{err}");

            return Err(ValidateTicketError::XMLParsingError);
        }
    };

    let response = ServiceResponse::from_str(&content).unwrap();

    match response.authentication_success {
        Some(_) => Ok(response),
        None => Err(ValidateTicketError::AuthenticationFailed),
    }
}
