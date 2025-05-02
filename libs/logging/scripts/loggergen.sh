set -eu -o pipefail

BASE_DIR=$(cd $(dirname $0); pwd)
PROJ_DIR=$(cd $BASE_DIR/../../..; pwd)
TOOLS_BIN=$PROJ_DIR/tools/bin

OP='codegen'

for OPT in "$@"
do
  case $OPT in
    '--rm')
      OP='rm'
      shift
      ;;
  esac
done

deno run -q --allow-read=$PROJ_DIR $BASE_DIR/targets.js | \
  jq -c '.targets[]' | \
  while read -r JSON
  do
    LOGGER_RS=$(echo "$JSON" | jq -r 'select(.loggerPath != null) | .loggerPath')
    if [ -z "$LOGGER_RS" ]
    then
      continue
    fi

    if [ "$OP" = 'rm' ]
    then
      rm -f $LOGGER_RS
      continue
    fi

    echo "$JSON" | \
      deno run -q \
        --allow-read=$BASE_DIR/logger.rs.hbs \
        $TOOLS_BIN/codegen.js --input-stdin --no-escape $BASE_DIR/logger.rs.hbs | \
      rustfmt --emit=stdout >$LOGGER_RS
  done
