# BEE

> Browser Engine for Embedding

[![ci-status](https://github.com/bee-browser/prototype/workflows/CI/badge.svg)](https://github.com/bee-browser/prototype/actions?workflow=CI)
[![codecov](https://codecov.io/gh/bee-browser/prototype/branch/master/graph/badge.svg?token=ZU1I8W30M9)](https://codecov.io/gh/bee-browser/prototype)

## What's BEE?

BEE is a web browser engine intended to be embedded into other applications.

## Development

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

# Update workflows for GitHub Actions.
make github-workflows
```

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
