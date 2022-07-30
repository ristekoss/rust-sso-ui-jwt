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

/// Gets a single organization based on the organization code.
///
/// # Options
///
/// Because an organization with the given organization code could possibly not exist, this
/// function simply returns an [`Option`] and leave how to handle the output to the user.
///
/// # Examples
///
/// ```rust
/// use sso_ui_jwt::orgs::{get_organization, Organization};
///
/// let org_code = "01.00.12.01";
/// let another_org_code = "Random org code";
///
/// assert_eq!(
///     Some(
///         Organization {
///             faculty: String::from("Ilmu Komputer"),
///             short_faculty: String::from("Fasilkom"),
///             major: String::from("Ilmu Komputer (Computer Science)"),
///             program: String::from("S1 Reguler (Undergraduate Program)"),
///         }
///     ),
///     get_organization(org_code),
/// );
///
/// assert_eq!(
///     None,
///     get_organization(another_org_code),
/// );
/// ```
pub fn get_organization(org_code: &str) -> Option<Organization> {
    let mut orgs = get_organizations();

    // `remove()` moves the value out of the `orgs` hash map so the value we return does not have a
    // dangling reference, making this code safe.
    orgs.remove(org_code)
}
