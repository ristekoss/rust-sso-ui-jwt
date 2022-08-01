//! Ticket validation errors.

/// Types of errors that could happen in ticket validation.
#[derive(Debug)]
pub enum ValidateTicketError {
    /// Failed ticket authentication.
    AuthenticationFailed,
    /// Errors regarding the validation request.
    RequestError,
    /// Error parsing the XML response.
    XMLParsingError,
}
