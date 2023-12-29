# bee-logview

> A tool to view logs in a web UI

## How does it work?

`bee-logview` starts a small web server for the web application to graphically render JSON logs
coming from a command spawned by `bee-logview`.

A typical usage is like this:

```shell
cargo run -r -p bee-logview -- -c jsparser/logview/config.yaml \
  -d url=https://cdnjs.cloudflare.com/ajax/libs/react/18.2.0/umd/react.production.min.js
```

`config.yaml`:

```yaml
mounts:
  - target: /logview
    source:
      fs: assets

event-source:
  command: >-
    sh run.sh {{url}}
```

Once performing the above command, a tab (or window) will be created in the default web browser and
the content of the web application will be rendered on it.

`STDIN` and `STDOUT` of `bee-logview` will be piped to ones of the child process.  And `STDERR` of
the child process will be used as an *event source* handled by the web application.
