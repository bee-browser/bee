# BEE

> Browser Engine for Embedding

[![ci-status](https://github.com/bee-browser/prototype/workflows/CI/badge.svg)](https://github.com/bee-browser/prototype/actions?workflow=CI)
[![coverage](https://coveralls.io/repos/github/bee-browser/prototype/badge.svg?branch=master)](https://coveralls.io/github/bee-browser/prototype?branch=master)
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

## What's BEE?

BEE is a web browser engine intended to be embedded into other applications.

## Development

Install `cargo-make`.  Other tools required will be installed automatically when running
`cargo make <task>`.

```shell
# Build the debug binary.
cargo make build

# Build the release binary.
cargo make build-release

# Run tests.
cargo make test

# Run coverage tests using grcov.
cargo make coverage-grcov

# Launch VSCode.
# The Linux build using a remote container is supported.
cargo make launch-vscode

# Update workflows for GitHub Actions.
cargo make update-github-workflows
```

You can see all steps by running `cargo make --list-all-steps`.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License
  ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
