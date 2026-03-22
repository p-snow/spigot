# Socket HTTP server


## Coding Standards

This project enforces strict adherence to standard Rust coding styles and idiomatic practices. All AI agents must comply with the following guidelines when generating or modifying code:

- *Mandatory =rustfmt= Compliance*
  - Every =.rs= file created or modified must be formatted according to the project's =rustfmt.toml= (or the default =rustfmt= configuration).
  - Manual formatting is not permitted; the output of =rustfmt= is the sole source of truth.
  - Before committing or submitting any changes, you must ensure that running =cargo fmt --all= results in no further changes to the codebase.

- *Rust API Guidelines & Idiomatic Rust*
  - All code must align with the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
  - Prioritize readability, memory safety, and idiomatic patterns over imperative or C-style logic.

- *CI/CD Integration*
  - The project utilizes =cargo fmt --check= in the CI/CD pipeline.
  - Any code failing this check will be rejected. AI agents are responsible for verifying compliance before submitting PRs.

- *Verification Checklist*
  - Before finalizing any code generation, perform the following verification steps:
    - *Formatting:* =cargo fmt --all -- --check=
    - *Linting:* =cargo clippy --all-targets --all-features -- -D warnings=

## Testing

**Run tests after every code modification**: `make test`
