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
# Build 'bjs' before running the following commands.

# Use //target/release/bjs.
test262 --test262-dir=path/to/tc39/test262 launch /bin/sh launchers/bjs.sh

# Use //target/debug/bjs.
test262 --test262-dir=path/to/tc39/test262 launch -- /bin/sh launchers/bjs.sh --debug
```

This command is slower than the previous command.  However, the `test262` process will perform all
the tests even if some of the tests cause panics.

The both commands will output the test results to STDOUT in the JSON format.

```json
{
  "timestamp": 1759840249814,
  "results": [
    {
      "file": "built-ins/WeakRef/prop-desc.js",
      "strict": false,
      "status": "failed",
      "duration": {
        "secs": 0,
        "nanos": 12911579
      },
      "metadata": {
        "description": "Property descriptor of WeakRef\n",
        "info": "...",
        "author": null,
        "esid": "sec-weak-ref-constructor",
        "es5id": null,
        "es6id": null,
        "negative": null,
        "includes": [
          "propertyHelper.js"
        ],
        "flags": [],
        "locale": [],
        "features": [
          "WeakRef"
        ]
      }
    },
    ...
  ]
}
```

## Tips

Run the tests one by one:

```shell
RAYON_NUM_THREAD=1 test262 --test262-dir=path/to/tc39/test262 run
```
