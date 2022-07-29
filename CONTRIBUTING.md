# Contributing

We are happy to welcome you on board this project 🎉

One of our goals is to promote the open-source culture and contributions are a big part of open-source development.
The contributions you make will be listed on the [`CHANGELOG.md`](https://github.com/ristekoss/rust-sso-ui-jwt/tree/main/CHANGELOG.md) of the next release.

## Issues

You can view, open, or comment on issues in this project [here](https://github.com/ristekoss/rust-sso-ui-jwt/issues)

### Bug Reports

To report a bug issue, if possible please include the following details as it would help us in troubleshooting:

> - The `sso-ui-jwt` version installed
> - The version of rust that you use
> - Your OS name and version
> - Backtrace logs
> - Details about your local setup which might help
> - Steps to reproduce the bug

### Feature Requests

If you plan on proposing a feature in an issue:

- Provide a detailed and descriptive explanation of the feature
- Keep the scope as narrow as possible for easy implementation
- If possible, try to provide a code sample of what it would look like

Because this project is open-sourced, we also welcome your contribution in code, documentation, anything really ✌️

## Code Style and Linting

Our code standards are enforced in style using [`rustfmt`](https://github.com/rust-lang/rustfmt) 
and linted with [`clippy`](https://github.com/rust-lang/rust-clippy). 
Before committing, make sure to run `cargo fmt --all` to format your code as PRs with unformatted code will not be accepted.

## Unsafe Code

Code that defines or uses `unsafe` functions needs to be explained via comments.
`unsafe` code can potentially cause undefined behaviour related bugs and other kinds of bugs. This could lead towards a security vulnerability.
If you commit code containing `unsafe`, please confirm that its usage is necessary and correct.

## Code Documentation

This project follows the documentation standards as explained in the [`rustdoc`](https://doc.rust-lang.org/stable/rustdoc/) book.
For more information, we recommend you to read the book and start from there.

## Pull Requests

You can comment on an existing issue if you'd like to work on it.
If you have a feature in mind, please open an issue first before working on it yourself.
The proposed feature must be determined first on whether or not it is something that is inline with the project and the community.

In your PR, you should:

1. Pass the tests for combinations of features and all features
2. Pass the code style and lint checks
3. Update the documentation and example when new functionality is added or modified
4. Add a note to the [`CHANGELOG.md`](https://github.com/ristekoss/rust-sso-ui-jwt/tree/main/CHANGELOG.md) about the changes
5. Add your name to the [`CHANGELOG.md`](https://github.com/ristekoss/rust-sso-ui-jwt/tree/main/CHANGELOG.md) for credit

## Commit Messages

When you create a commit message, it should describe the action that is done by the commit's changes in present tense.

Examples:
- `Implement token validation handler` - The commit implements part of a feature
- `Fix xml parsing EOF bug` - Fixes an unintended behaviour

Not examples:
- `Changed ticket validation endpoint` - Used past tense
- `Token module updates` - No action verb and ambiguous meaning

------------------------------------------------------------------------------------------------------

Once again, welcome on board 😁
