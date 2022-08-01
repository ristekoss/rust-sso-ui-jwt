# rust-sso-ui-jwt

Rust library for JWT utilities from SSO UI

Universitas Indonesia (UI) uses a single sign on (SSO) mechanism
for authentication and authorization of their services.
The response body to a successful sign on is written in XML. It is
generally a chore to reimplement the handling of entire SSO ticket validation and response
body processing for students, committees, and organizations who'd like to use UI's SSO for
student account authentication and authorization in their events.

That's why we decided to make this library.
It parses the response data into a JSON format and we provide
the SSO ticket validation mechanism as well as JWT utilities for the data.

## Documentation

You can view the project's documentation in it's [docs.rs](https://docs.rs/sso-ui-jwt/latest/sso_ui_jwt) website.

## Usage

See the [examples](https://github.com/ristekoss/rust-sso-ui-jwt/tree/main/examples) for reference on using this library.

## Installation

Add this to your `Cargo.toml` file:

```toml
[dependencies]
sso-ui-jwt = "0.4"
```

## Features

Enabling or disabling features can be done by configuring the library from `Cargo.toml`:

```toml
[dependencies.sso-ui-jwt]
version = "0.4"
features = ["log"]
```

As of right now, there are no default features implemented.

Full list of features:

- **`log`**: Logs the messages within the library

## Contributing

If you'd like to contribute to this project, please read our [contributing guidelines](https://github.com/ristekoss/rust-sso-ui-jwt/tree/main/CONTRIBUTING.md) **before creating pull requsts**.

## Maintainers

This project is currently maintained by the following members of RISTEKOSS:

- [Nayyara Airlangga](https://github.com/nayyara-airlangga)
