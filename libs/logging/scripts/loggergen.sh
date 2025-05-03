set -eu -o pipefail

BASE_DIR=$(cd $(dirname $0); pwd)
PROJ_DIR=$(cd $BASE_DIR/../../..; pwd)
TOOLS_BIN=$PROJ_DIR/tools/bin

OP="$1"

case $1 in
  'codegen')
    deno run -q --allow-read=$BASE_DIR/logger.rs.hbs \
      $TOOLS_BIN/codegen.js --input-inline --no-escape $BASE_DIR/logger.rs.hbs "$2" | \
      rustfmt --emit=stdout
    ;;
  'deps')
    echo "$BASE_DIR/loggergen.sh $BASE_DIR/logger.rs.hbs"
    ;;
esac
