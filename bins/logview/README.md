# logview

> A tool to view logs in a web UI

## How does it work?

`logview` starts a small web server for the web application to graphically render JSON logs coming
from a command spawned by `logview`.

A typical usage is like this:

```shell
cargo run -rqp logview -- -c examples/hello/config.yaml -d name=world

curl https://cdnjs.cloudflare.com/ajax/libs/react/18.2.0/umd/react.production.min.js -sG | \
  cargo run -rqp logview -- -c ../../libs/jsparser/logview/config.yaml
```

Once performing the above command, a tab (or window) will be created in the default web browser and
the content of the web application will be rendered on it.

`STDIN` and `STDOUT` of `logview` will be piped to ones of the child process.  And `STDERR` of the
child process will be used as an *event source* handled by the web application.
