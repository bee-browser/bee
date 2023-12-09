# bee-estree

> A simple Javascript parser to output ESTree in JSON

This package provides a binary crate `bee-estree` to output the [ESTree] of the input JavaScript
program.

The main goal of this package is providing a tool for validating the implementation of
`bee-jsparser`.

## How to use

The following command parses a JavaScript program:

```shell
curl https://cdnjs.cloudflare.com/ajax/libs/react/18.2.0/umd/react.production.min.js -sG |
  bee-estree parse script | jq
```

This shows the ESTree representation of the JavaScript program.  The representation is compatible
with [Acorn].

The following command starts a server that responds to requests to parse JavaScript programs:

```shell
cat | bee-estree serve | jq
```

The server can accept requests like below:

```json5
{
  "sourceType": "script",  // "script" or "module".
  "source": "1"            // A source text to be parsed.
}
```

and output responses like below:

```json5
{
  // The ESTree representation of the JavaScript program.
  "program": {
    "type": "Program",
    "start": 0,
    "end": 1,
    "body": [
      {
        "type": "ExpressionStatement",
        "start": 0,
        "end": 1,
        "expression": {
          "type": "Literal",
          "start": 0,
          "end": 1,
          "value": 1,
          "raw": "1"
        }
      }
    ],
    "sourceType": "script"
  },
  // The elapsed time in nanoseconds.
  "elapsed": 71925
}
```

## Semantic action for each production rule

See [src/builder/actions.yaml](./src/builder/actions.yaml).  This file defines a semantic action
for each production rule.  Each action itself is implemented as an instance method of `Builder`.

## Validating output

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

## Tips

### jq: parse error: Exceeds depth limit for parsing

`jq` may not be able to parse an ESTree JSON due to a limitation on the depth.  For example:

```shell
curl https://cdnjs.cloudflare.com/ajax/libs/typescript/5.3.3/typescript.min.js -sG | \
  bee-estree parse script | jq
```

This command causes the following error:

```
jq: parse error: Exceeds depth limit for parsing at line 1, column 10759553
Error: Broken pipe (os error 32)
```

In this case, use other JSON parser commands such as [`jj`]:

```
curl https://cdnjs.cloudflare.com/ajax/libs/typescript/5.3.3/typescript.min.js -sG | \
  bee-estree parse script | jj -p
```

## TODO

* JSON doesn't support BigInt
  * https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#use_within_json
  * Switch to [JSON5]?
* Generate the `Builder` implementation from a definition of semantic actions
  * Currently, it's manually implemented

## Acknowledgments

[ESTree] is a famous specification for AST representation.

[Acorn] is used for validating our implementation.

[AST Explorer] is one of useful web applications for learning AST representations of existing
JavaScript parsers.

[tc39/test262] is official ECMAScript conformance test suite.

[ESTree]: https://github.com/estree/estree
[Acorn]: https://www.npmjs.com/package/acorn
[AST Explorer]: https://astexplorer.net/
[tc39/test262]: https://github.com/tc39/test262
[JSON5]: https://github.com/json5/json5
[`jj`]: https://github.com/tidwall/jj
