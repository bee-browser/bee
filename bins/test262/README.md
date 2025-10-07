# test262

> A test262 runner for `jsruntime`

## Usage

Perform the tests in threads:

```shell
test262 --test262-dir=path/to/tc39/test262 run
```

This command is faster than others.  However, the `test262` process crashes before the all the
tests are completed if a test causes a panic due to a bug.

Perform the tests in processes:

```shell
# Use the release binary.
test262 --test262-dir=path/to/tc39/test262 launch /bin/sh launcher/bjs.sh

# Use the debug binary.
test262 --test262-dir=path/to/tc39/test262 launch -- /bin/sh launcher/bjs.sh --debug
```

This command is slower than the previous command.  However, the `test262` process will perform all
the tests even if some of the tests cause panics.  The test status of a test causing a panic is set
to `aborted`.

The both commands will output the test results to STDOUT in a JSON format.

## Tips

Run the tests one by one:

```shell
RAYON_NUM_THREAD=1 test262 --test262-dir=path/to/tc39/test262 run
```
