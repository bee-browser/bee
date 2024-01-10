# logging

> A logging system with fast log filters

## Environment variables

For the time being, `RUST_LOG` must be defined like below:

```shell
RUST_LOG=info,bee=trace
```

No log will be shown by default.  `BEE_LOG_DEFAULT` sets the default log flags:

```shell
# Turn off logging (default).
BEE_LOG_DEFAULT=off

# Show `error`, `warn` and `info` logs.
BEE_LOG_DEFAULT='error|warn|info'
```

The following flags are defined:

  * `error`
  * `warn`
  * `info`
  * `debug0`
  * `debug1`
  * `debug2`
  * `trace`

Unlike log levels used in `RUST_LOG`, these are **flags**, not levels.  So, `BEE_LOG_DEFAULT=info`
shows only `info` logs.

In addition, the following values can be specified:

  * `off`
  * `all`
  * `debug` (`debug0|debug1|debug2`)

The log flags specified in `BEE_LOG_DEFAULT` are applied to all targets.  `BEE_LOG` sets log flags
for each target:

```shell
# A filter without target works as the default log flags.
BEE_LOG='error|warn|info'

# The `bee` target works as the default log flags for our modules.
# All targets of logs from our modules start with the `bee::` prefix.
BEE_LOG='bee=error|warn'

# The `bee::estree` target shows `debug0`, `debug1` and `debug2` logs additionally.
BEE_LOG='bee::estree+=debug'

# The `bee::jsparser` target shows only `trace` logs.
# Its child targets such as `bee::jsparser::lexer` also show only `trace` logs.
BEE_LOG='bee::jsparser=trace'

# Multiple filters can be specified in CSV:
BEE_LOG='bee::estree+=debug,bee::jsparser=trace'
```

Logs are shown in a human-readable text format by default.  `BEE_LOG_FORMAT` changes the format:

```shell
# Show logs in a human-readable text format (default).
BEE_LOG_FORMAT=text

# Show logs in JSON (NDJSON).
BEE_LOG_FORMAT=json
```

Logs are shown with uptime as timestamp by default.  `BEE_LOG_TIMESTAMP` changes the timestamp
type:

```shell
# Uptime with micro-second precision (default).
RUST_LOG_TIMESTAMP=uptime

# Local datetime in RFC3339 with micro-second precision.
RUST_LOG_TIMESTAMP=local

# No timestamp is shown.
RUST_LOG_TIMESTAMP=off
```

## How to add a new target

Create `logging.yaml` in a package root:

```shell
# //bins/estree/logging.yaml
targets:
  - bee::estree
```

Run the following commands:

```shell
make codegen

# Run in the project root
make -C ../.. loggergen
```

`make codegen` will update [src/targets.rs](./src/targets.rs) and `make loggergen` will update
`logger.rs` in each target module.

Finally, add `mod logger;` in `main.rs`, `lib.rs` or `mod.rs`.  The logging macro functions can be
accessible via `logger` like below:

```rust
mod logger; // or `use super::logger;` in a child module.

fn do_something() {
  logger::info!("hi there");
}
```

## Why are `logger.rs` files needed?

This is some kind of workaround.

We use a `logger.rs` file in order to convert logging target name (module path) into an integer
identifier before execution.  Initially, we expected to use a constant function for this purpose,
but we soon knew that this is not possible at this time.

```rust
// We expected that `logging::target()` can be used in `logging::info!()` macro like below:
//
//   if logging::target(module_path!()).info_enabled() {
//       logging::imp::info(target: concat!("bee::", module_path!()), $($tokens)+);
//   }
//
// But we cannot because...
pub const fn target(name: &'static str) -> &'static Target {
    match name {
        // error[E0015]: cannot match on `str` in constant functions
        "estree" => ESTREE,
        ...
    }
}
```

## TODO

* [ ] Implement own backend
  * Currently, [`tracing`] and [`tracing-subscriber`] are used
