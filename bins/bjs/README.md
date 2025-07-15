# bjs

> A command-line tool for testing `jsruntime`

## Usage

Run JavaScript programs:

```shell
bjs run program.js

# or
cat program.js | bjs run

# multiple files can be specified
bjs run a.js b.js
```

Show [Cranelift IR]:

```shell
bjs compile program.js
```

Show [CFG]:

```shell
bjs print-cfg program.js | xdot -
```

All commands can be shown by `bjs -h`.

## tc39/test262

```shell
make build OPTIONS=-r
time sh bins/bjs/scripts/test262.sh --progress >ctrf.json
```

After about 10 minutes, you can see results like this:

```
96432 tests: 3571 passed, 0 skipped, 67720 aborted, 4 timed-out, 25137 failed

real    9m22.444s
user    4m56.751s
sys     3m59.425s
```

The test results are output to STDOUT in the [CTRF] format.

[Cranelift IR]: https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/docs/ir.md
[CFG]: https://en.wikipedia.org/wiki/Control-flow_graph
[CTRF]: https://ctrf.io/
