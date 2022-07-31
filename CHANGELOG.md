# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

## [Unreleased]

### Added
- Create code style workflow ([#10](https://github.com/ristekoss/rust-sso-ui-jwt/pull/10)) ([@nayyara-airlangga])

## Removed
- Remove `BadRequest` from `ValidateTicketError` ([#11](https://github.com/ristekoss/rust-sso-ui-jwt/pull/11)) ([@nayyara-airlangga])

### Fixed
- Fix `actix-web-example` to match the code standards ([#10](https://github.com/ristekoss/rust-sso-ui-jwt/pull/10)) ([@nayyara-airlangga])
- Fix `ValidateTicketError` for xml response parsing ([#11](https://github.com/ristekoss/rust-sso-ui-jwt/pull/11)) ([@nayyara-airlangga])


## [v0.2.0] - 2022-07-30

Thanks to the following for their contributions:
- [@nayyara-airlangga]

### Added
- Add code documentation ([#3](https://github.com/ristekoss/rust-sso-ui-jwt/pull/3)) ([@nayyara-airlangga])
- Create SSO user struct ([#4](https://github.com/ristekoss/rust-sso-ui-jwt/pull/4)) ([@nayyara-airlangga])
- Add get single organization handler ([#6](https://github.com/ristekoss/rust-sso-ui-jwt/pull/6)) ([@nayyara-airlangga])

### Changed
- Reduce feature helper modules visibility to `pub(super)` ([#5](https://github.com/ristekoss/rust-sso-ui-jwt/pull/5)) ([@nayyara-airlangga])
- Remove `Clone` and `PartialEq` trait implementation from the `Organization` struct ([#6](https://github.com/ristekoss/rust-sso-ui-jwt/pull/6)) ([@nayyara-airlangga])


## [v0.1.1] - 2022-07-29

Thanks to the following for their contributions:
- [@nayyara-airlangga]

### Added
- Add [`CONTRIBUTING.md`][contributing.md] ([@nayyara-airlangga])


## [v0.1.0] - 2022-07-29

Thanks to the following for their contributions:
- [@nayyara-airlangga]

### Added
- Release initial version of the project ([@nayyara-airlangga])


<!---------- LINKS ---------->
[Keep A Changelog]: https://keepachangelog.com/en/1.0.0
[Semantic Versioning]: https://semver.org
[contributing.md]: https://github.com/ristekoss/rust-sso-ui-jwt/tree/main/CONTRIBUTING.md

<!-- VERSION COMPARISON -->
[Unreleased]: https://github.com/ristekoss/rust-sso-ui-jwt/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/ristekoss/rust-sso-ui-jwt/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/ristekoss/rust-sso-ui-jwt/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/ristekoss/rust-sso-ui-jwt/tree/v0.1.0

<!-- AUTHORS -->
[@nayyara-airlangga]: https://github.com/nayyara-airlangga
