# bjs

> A command-line tool for testing `jsruntime`

## Usage

```shell
bjs run program.js

# or
cat program.js | bjs run
```

All commands can be shown by `bjs -h`.

## tc39/test262

```shell
make build OPTIONS=-r
time sh bins/bjs/scripts/test262.sh --progress
```

After about 10 minutes, you can see results like this:

```
96432 tests: 3571 passed, 0 skipped, 67720 aborted, 4 timed-out, 25137 failed

real    9m22.444s
user    4m56.751s
sys     3m59.425s
```

TODO:

* [ ] Output results in [CTRF]
* [ ] Generate a graphical report from CTRF

[CTRF]: https://ctrf.io/
