use serde::Serialize;
use strong_xml::XmlRead;

#[derive(Serialize, XmlRead)]
#[xml(tag = "cas:serviceResponse")]
pub struct ServiceResponse {
    #[xml(child = "cas:authenticationSuccess")]
    pub authentication_success: Option<AuthenticationSuccess>,
}

#[derive(Serialize, XmlRead)]
#[xml(tag = "cas:authenticationSuccess")]
pub struct AuthenticationSuccess {
    #[xml(flatten_text = "cas:user")]
    pub user: String,

    #[xml(child = "cas:attributes")]
    pub attributes: Attributes,
}

#[derive(Serialize, XmlRead)]
#[xml(tag = "cas:attributes")]
pub struct Attributes {
    #[xml(flatten_text = "cas:ldap_cn")]
    pub ldap_cn: String,

    #[xml(flatten_text = "cas:kd_org")]
    pub kd_org: String,

    #[xml(flatten_text = "cas:peran_user")]
    pub peran_user: String,

    #[xml(flatten_text = "cas:nama")]
    pub nama: String,

    #[xml(flatten_text = "cas:npm")]
    pub npm: String,
}
