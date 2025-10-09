# tools/bin

This folder contains scripts files implementing commands used in development.

There are two types of scripts:

1. Shell scripts (`*.sh`) which can be run with `sh` or `bash`
2. JavaScript scripts (`*.js`) which can be run with `deno`

You can choose either one depending on the task that the script does.

Every command supports the `-h` and `--help` options.  The help of each command is described in
accordance with the [`docopt`] format like this:

```
Run a command for a performance measurement.

USAGE:
  $PROGNAME [--addr2line <addr2line>] -- <command>...
  $PROGNAME -h | --help

OPTIONS:
  --addr2line <addr2line> [default: $ADDR2LINE]
    Path to addr2line.  gimli-rs/addr2line is used by default.

    addr2line in binutils is very slow.  See flamegraph-rs/flamegraph#74.  Typically, we have to
    wait several minutes for results to be processed when measuring performance with
    'cargo flamegraph'.

ARGUMENTS:
  <command>
    A command to perform.

DESCRIPTION:
  This script changes /proc/sys/kernel/perf_event_paranoid temporarily and perform the specifed
  command for a performance measurement.

EXAMPLES:
  $PROGNAME -- cargo flamegraph --bin=bjs --profile=release-symbols -- \\
    run libs/jsruntime/benches/dataset/fib32.js
```

The help starts with a single-line short description of the command, followed by sections.

Each section starts with a line in the format `<UPPERCASE-SECTION-NAME>:`.  The `USAGE:` section is
always required and others are optional.

[`docopt`]: http://docopt.org/
