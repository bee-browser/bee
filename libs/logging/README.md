# logging

> A logging system with fast log filters

## Environment variables

Use `RUST_LOG` for filtering logs:

```shell
RUST_LOG=info,bee=debug
```

No log will be shown by default.

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
BEE_LOG_TIMESTAMP=uptime

# Local datetime in RFC3339 with micro-second precision.
BEE_LOG_TIMESTAMP=local

# No timestamp is shown.
BEE_LOG_TIMESTAMP=off
```

## How to use logger

Use `logging::define_logger` macro like this:

```rs
// Define a logger whose logging target is `std::module_path!()`.
logging::define_logger {}

// Or explicitly specify the logging target in the `TypePath` form.
logging::define_logger { bee::target_name }

fn do_something() {
  logger::info!("hi there");
}

mod sub_module {
  // Before using logging macros, it's necessary to define a new logger inside the module,
  // or import an existing logger from outside the module.
  use super::logger;

  fn do_something() {
    logger::info!("hi there");
  }
}
```

Don't rename the logger when importing it.

## TODO

* [ ] Implement own backend
  * Currently, [`tracing`] and [`tracing-subscriber`] are used
