//! Structs that represents the parsed XML response.

use serde::Serialize;
use strong_xml::XmlRead;

/// A service response tag from the CAS SSO server.
#[derive(Clone, Serialize, XmlRead)]
#[xml(tag = "cas:serviceResponse")]
pub struct ServiceResponse {
    /// A successful authentication tag. Can be `None` if authentication failed.
    #[xml(child = "cas:authenticationSuccess")]
    pub authentication_success: Option<AuthenticationSuccess>,
}

/// A successful authentication XML tag.
#[derive(Clone, Serialize, XmlRead)]
#[xml(tag = "cas:authenticationSuccess")]
pub struct AuthenticationSuccess {
    /// A user tag containing the authenticated username.
    #[xml(flatten_text = "cas:user")]
    pub user: String,

    /// The user's atrributes.
    #[xml(child = "cas:attributes")]
    pub attributes: Attributes,
}

/// Represents a user's attributes XML tag.
#[derive(Clone, Serialize, XmlRead)]
#[xml(tag = "cas:attributes")]
pub struct Attributes {
    /// The LDAP common name of the user.
    #[xml(flatten_text = "cas:ldap_cn")]
    pub ldap_cn: String,

    /// User's organization code.
    #[xml(flatten_text = "cas:kd_org")]
    pub kd_org: String,

    /// User's role
    #[xml(flatten_text = "cas:peran_user")]
    pub peran_user: String,

    /// User's name
    #[xml(flatten_text = "cas:nama")]
    pub nama: String,

    /// User's student ID number
    #[xml(flatten_text = "cas:npm")]
    pub npm: String,
}
