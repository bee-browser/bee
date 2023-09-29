# BEE

> Browser Engine for Embedding

[![ci-status](https://github.com/bee-browser/prototype/workflows/ci/badge.svg)](https://github.com/bee-browser/prototype/actions?workflow=ci)
[![codecov](https://codecov.io/gh/bee-browser/prototype/branch/main/graph/badge.svg?token=ZU1I8W30M9)](https://codecov.io/gh/bee-browser/prototype)

## What's BEE?

BEE is a web browser engine intended to be embedded into other applications.

## Development

Install the following software:

* [Rust]
* [GNU Make]
* [Deno]
* [Nextest]
* [grcov]

Then:

```shell
# Release build
make build
make test

# Debug build
make debug-build
make debug-test

# Make a test coverage report
make coverage
make coverage-html

# Make docs
make doc

# List targets defined in the top-level Makefile
make list
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

[Rust]: https://www.rust-lang.org/
[GNU Make]: https://www.gnu.org/software/make/
[Deno]: https://deno.com/
[Nextest]: https://github.com/nextest-rs/nextest
[grcov]: https://github.com/mozilla/grcov
[LICENSE-APACHE]: ./LICENSE-APACHE
[LICENSE-MIT]: ./LICENSE-MIT
