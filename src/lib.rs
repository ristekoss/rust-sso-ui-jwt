//! Rust library for JWT utilities from SSO UI.
//!
//! Universitas Indonesia (UI) uses a single sign on (SSO) mechanism
//! for authentication and authorization of their services.
//! The response body to a successful sign on is written in XML. It is
//! generally a chore to reimplement the handling of entire SSO ticket validation and response
//! body processing for students, committees, and organizations who'd like to use UI's SSO for
//! student account authentication and authorization in their events.
//!
//! That's why decided to make this library.
//! It parses the response data into a JSON format and we provide
//! the SSO ticket validation mechanism as well as JWT utilities for the data.
//!
//! # Usage
//!
//! See the [examples] for reference on this library's usage.
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! sso-ui-jwt = "0.4"
//! ```
//!
//! # Features
//!
//! Enabling or disabling features can be done by configuring the library from `Cargo.toml`:
//!
//! ```toml
//! [dependencies.sso-ui-jwt]
//! version = "0.4"
//! features = ["log"]
//! ```
//!
//! As of right now, there are no default features implemented.
//!
//! Full list of features:
//!
//!- **`log`**: Logs the messages within the library
//!
//! [examples]: https://github.com/ristekoss/rust-sso-ui-jwt/tree/main/examples

pub mod config;
pub mod orgs;
pub mod ticket;
pub mod token;

pub use config::SSOJWTConfig;
