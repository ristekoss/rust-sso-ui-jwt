# rust-sso-ui-jwt

Rust library for JWT utilities from SSO UI

## Usage

See the [examples](https://github.com/ristekoss/rust-sso-ui-jwt/tree/main/examples) for reference on using this library.

## Installation

Add this to your `Cargo.toml` file:

```toml
[dependencies]
sso-ui-jwt = "0.1.0"
```

## Features

Enabling or disabling features can be done by configuring the library from `Cargo.toml`:

```toml
[dependencies.sso-ui-jwt]
version = "0.1.0"
features = ["log"]
```

As of right now, there are no default features implemented.

Full list of features:

- **log**: Enabling the `log` feature will enable the library to log messages within the library

## Maintainers

This project is currently maintained by the following members of RISTEKOSS:

- [Nayyara Airlangga](https://github.com/nayyara-airlangga)
