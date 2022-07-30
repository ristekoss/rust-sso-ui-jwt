//! UI's organization data and helpers.
//!
//! Organization in this context means data regarding to a student's faculty, short faculty name,
//! major, and the program/degree they are pursuing.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod orgcode;
use orgcode::ORG_CODES;

/// Represents an academic organization in UI.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Organization {
    /// Name of the organization's faculty.
    pub faculty: String,
    /// Abbreviation of the faculty name.
    #[serde(alias = "shortFaculty")]
    pub short_faculty: String,
    /// Major of the organization.
    pub major: String,
    /// Educational program of the organization.
    pub program: String,
}

/// Fetches all organizations.
///
/// # Examples
///
/// ```rust
/// use sso_ui_jwt::orgs::{get_organizations, Organization};
///
/// let orgs = get_organizations();
/// let org = orgs.get("01.00.12.01").unwrap();
///
/// assert_eq!(org, &Organization {
///     faculty: String::from("Ilmu Komputer"),
///     short_faculty: String::from("Fasilkom"),
///     major: String::from("Ilmu Komputer (Computer Science)"),
///     program: String::from("S1 Reguler (Undergraduate Program)"),
/// });
/// ```
pub fn get_organizations() -> HashMap<String, Organization> {
    serde_json::from_str(ORG_CODES).unwrap()
}
