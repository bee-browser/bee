BASE_DIR=$(cd $(dirname $0); pwd)
PROJ_DIR=$(cd $BASE_DIR/../../..; pwd)
deno run -q \
  --allow-env \
  --allow-run \
  --allow-read=$PROJ_DIR \
  $BASE_DIR/test262_parser_tests.js $@
