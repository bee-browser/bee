PROGNAME=$(basename $0)
BASE_DIR=$(cd $(dirname $0); pwd)
PROJ_DIR=$(cd $BASE_DIR/../../..; pwd)

help() {
  cat <<EOF >&2
Launch bjs.

USAGE:
  $PROGNAME [options] <program>
  $PROGNAME -h | --help

OPTIONS:
  --debug
    Launch a debug executable.

  --script
    Evaluate <program> as a script.

  --module
    Evaluate <program> as a module.

  --harness <harness>
    Path to the harness file.

ARGUMENTS:
  <program>
    Path to the program file.
EOF
  exit 0
}

BJS="$PROJ_DIR/target/release/bjs"
GLOBAL_OPTIONS=
RUN_OPTIONS=

while [ $# -gt 0 ]
do
  case "$1" in
    '-h' | '--help')
      help
      ;;
    '--debug')
      BJS="$PROJ_DIR/target/debug/bjs"
      shift
      ;;
    '--script')
      GLOBAL_OPTIONS="--as=script"
      shift
      ;;
    '--module')
      GLOBAL_OPTIONS="--as=module"
      shift
      ;;
    '--harness')
      RUN_OPTIONS="$RUN_OPTIONS --preload-scripts=$2"
      shift 2
      ;;
    *)
      break
      ;;
  esac
done

if [ ! -e "$BJS" ]
then
  exit 101
fi

exec $BJS $GLOBAL_OPTIONS run $RUN_OPTIONS $@
