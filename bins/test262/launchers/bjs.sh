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

  --strict
    Evaluate <program> in the strict mode.

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
STRICT=
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
    '--strict')
      STRICT=1
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

CONTENT=$(mktemp)
trap "rm -f $CONTENT" EXIT INT TERM

if [ "$STRICT" = 1 ]
then
  echo 'use strict;' >$CONTENT
fi
cat $@ >>$CONTENT

exec $BJS $GLOBAL_OPTIONS run $RUN_OPTIONS $CONTENT
