# BEE

> Browser Engine for Embedding

[![ci-status](https://github.com/bee-browser/prototype/workflows/ci/badge.svg)](https://github.com/bee-browser/prototype/actions?workflow=ci)
[![codecov](https://codecov.io/gh/bee-browser/prototype/branch/main/graph/badge.svg?token=ZU1I8W30M9)](https://codecov.io/gh/bee-browser/prototype)

## What's BEE?

BEE is a web browser engine intended to be embedded into other applications.

## Development

Install the following software:

* Rust
  * stable
  * nightly (used for generating a coverage report of tests)
* deno 1.30
* GNU make

Then:

```shell
# The release build.
make build
make test

# The debug build
make debug-build
make debug-test

# Coverage.
make coverage
make coverage-html
```

Worked only on Linux and macOS at this point.  Windows will be supported in the future.

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

[LICENSE-APACHE]: ./LICENSE-APACHE
[LICENSE-MIT]: ./LICENSE-MIT
