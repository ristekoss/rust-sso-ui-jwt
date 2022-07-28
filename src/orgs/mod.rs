use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Organization {
    pub faculty: String,

    #[serde(rename = "shortFaculty")]
    pub short_faculty: String,

    pub major: String,
    pub program: String,
}

pub fn get_organizations() -> HashMap<String, Organization> {
    let json = fs::read_to_string("./src/orgs/orgcode.json").unwrap();

    serde_json::from_str(&json).unwrap()
}
