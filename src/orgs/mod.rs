use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod orgcode;
use orgcode::ORG_CODES;

#[derive(Clone, Deserialize, Serialize)]
pub struct Organization {
    pub faculty: String,

    #[serde(rename = "shortFaculty")]
    pub short_faculty: String,

    pub major: String,
    pub program: String,
}

pub fn get_organizations() -> HashMap<String, Organization> {
    serde_json::from_str(ORG_CODES).unwrap()
}
