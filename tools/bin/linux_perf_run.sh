set -eu

PROGNAME=$(basename $0)
BASEDIR=$(realpath $(dirname $0))
PROJDIR=$(realpath $BASEDIR/../..)

ADDR2LINE="$PROJDIR/vendor/bin/addr2line"

help() {
  cat <<EOF >&2
Usage:
  $PROGNAME [--addr2line <addr2line>] -- <command>...
  $PROGNAME -h | --help

Options:
  -h, --help
    Show this screen.

  --addr2line <addr2line> [default: $ADDR2LINE]
    Path to addr2line.  gimli-rs/addr2line is used by default.

    addr2line in binutils is very slow.  See flamegraph-rs/flamegraph#74.
    Typically, we have to wait several minutes for results to be processed when
    measuring performance with "cargo flamegraph".

Arguments:
  <command>
    A command to perform.

Description:
  This script changes /proc/sys/kernel/perf_event_paranoid temporarily and
  perform the specifed command for a performance measurement.

Example:
  $PROGNAME -- cargo flamegraph --bin=jstb --profile=profiling -- eval >/dev/null
EOF
  exit 0
}

for OPT in "$@"
do
  case $OPT in
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
