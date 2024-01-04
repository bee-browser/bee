# BEE

> Browser Engine for Embedding

[![ci-status](https://github.com/bee-browser/bee/workflows/ci/badge.svg)](https://github.com/bee-browser/bee/actions?workflow=ci)
[![codecov](https://codecov.io/gh/bee-browser/bee/graph/badge.svg?token=ZU1I8W30M9)](https://codecov.io/gh/bee-browser/bee)

## What's BEE?

BEE is a web browser engine intended to be embedded into other applications.

## Development

Install the following software:

* [Rust]
* [GNU Bash]
* [GNU Make]
* [Deno]
* [Nextest]
* [grcov]
* [jq]
* [yq]

Then:

```shell
# Run tests
make test

# Make docs
make doc

# release build
make release-build

# List targets defined in the top-level Makefile
make list
```

Worked only on Linux and macOS at this point.  Windows will be supported in the future.

In addition, it's recommended to install the following software:

* [flamegraph]
* [perf]

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
[GNU Bash]: https://www.gnu.org/software/bash/
[GNU Make]: https://www.gnu.org/software/make/
[Deno]: https://deno.com/
[Nextest]: https://github.com/nextest-rs/nextest
[grcov]: https://github.com/mozilla/grcov
[jq]: https://jqlang.github.io/jq/
[yq]: https://mikefarah.gitbook.io/yq/
[flamegraph]: https://github.com/flamegraph-rs/flamegraph
[perf]: https://en.wikipedia.org/wiki/Perf_(Linux)
[LICENSE-APACHE]: ./LICENSE-APACHE
[LICENSE-MIT]: ./LICENSE-MIT
