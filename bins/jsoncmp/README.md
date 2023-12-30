# jsoncmp

> A tool to compare two JSON files

## Usages

```shell
cargo run -rqp jsoncmp -- lhs.json rhs.json
cat lhs.json | cargo run -rqp jsoncmp -- - rhs.json
cat rhs.json | cargo run -rqp jsoncmp -- lhs.json -
```

## Why not use existing tools?

We tried comparing large JSON files by using the following tools:

* [`json-diff`]
* [`json-structural-diff-cli`]

But these are very slow.

## Acknowledgments

We use [`assert-json-diff`] for the comparison.  It's fast, at least faster than the tools listed
above.

[`json-diff`]: https://www.npmjs.com/package/json-diff
[`json-structural-diff-cli`]: https://crates.io/crates/json-structural-diff-cli
[`assert-json-diff`]: https://crates.io/crates/assert-json-diff
