//! SSO ticket validation and XML response parsed structs.
//!
//! When a user has signed in, the CAS SSO server returns a ticket. That ticket is used as an
//! authorization mechanism to the CAS server and if valid will return an XML response that
//! contains the user data if successful and an error if it fails.

pub mod error;
pub mod handler;
pub mod payload;

pub use error::ValidateTicketError;
pub use handler::validate_ticket;
pub use payload::*;
