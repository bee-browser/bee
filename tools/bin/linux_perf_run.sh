set -eu

PROGNAME=$(basename $0)
BASEDIR=$(realpath $(dirname $0))
PROJDIR=$(realpath $BASEDIR/../..)

ADDR2LINE="$PROJDIR/vendor/bin/addr2line"

help() {
  cat <<EOF >&2
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
  $PROGNAME -- cargo flamegraph --bin=jstb --profile=release-symbols -- \\
    run libs/jsruntime/benches/dataset/fib32.js
EOF
  exit 0
}

while [ $# -gt 0 ]
do
  case "$1" in
    '-h' | '--help')
      help
      ;;
    '--addr2line')
      ADDR2LINE="$(realpath $2)"
      shift 2
      ;;
    '--')
      shift
      break
      ;;
  esac
done

CMD=$@

echo 'Change /proc/sys/kernel/perf_event_paranoid temporarily for the measurement'
SAVED=$(cat /proc/sys/kernel/perf_event_paranoid)
trap "echo $SAVED | sudo tee /proc/sys/kernel/perf_event_paranoid >/dev/null" EXIT
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid >/dev/null

echo "Running \`$CMD\`..."
export PATH="$(dirname $ADDR2LINE):$PATH"
$CMD
