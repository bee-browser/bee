# bee-estree

> A simple Javascript parser to output ESTree in JSON

This package provides a binary crate `bee-estree` to output the [ESTree] of the input JavaScript
program.

The main goal of this package is providing a tool for validating the implementation of
`bee-jsparser`.

## Semantic action for each production rule

See [src/builder/actions.yaml](./src/builder/actions.yaml).  This file defines a semantic action
for each production rule.  Each action itself is implemented as an instance method of `Builder`.

## Validating output with `acorn`

Run:

```shell
curl https://host/script.js -sG | sh ./scripts/validate.sh
```

Differences like below will be shown if the validation fails:

```text
json atoms at path ".body[0].end" are not equal:
    lhs:
        null
    rhs:
        9387
...
```

The paths shown in the above messages can be used as `jq` filters:

```shell
curl https://host/script.js -sG | cargo run | jq '.body[0].end'

# Show the parent node.
curl https://host/script.js -sG | cargo run | jq '.body[0]'
```

Debug-level logs are shown by specifying the `RUST_LOG` environment variable:

```shell
curl https://host/script.js -sG | RUST_LOG=debug cargo run >/dev/null
```

## tc39/test262

We have a test runner to test ECMAScript conformance using [tc39/test262]:

```shell
sh ./script/test262.sh
```

Many tests fails at the moment.  The `--details` option lists failed tests.

## Acknowledgments

[ESTree] is a famous specification for AST representation.

[acorn] is used for validating our implementation.

[AST Explorer] is one of useful web applications for learning AST representations of existing
JavaScript parsers.

[tc39/test262] is official ECMAScript conformance test suite.

[ESTree]: https://github.com/estree/estree
[acorn]: https://www.npmjs.com/package/acorn
[AST Explorer]: https://astexplorer.net/
[tc39/test262]: https://github.com/tc39/test262
