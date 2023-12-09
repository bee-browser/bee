BASE_DIR=$(realpath $(dirname $0))
PROJ_DIR=$(realpath $BASE_DIR/../../..)
TEST262_DIR=$PROJ_DIR/vendor/tc39/test262
deno run --allow-read=./,$TEST262_DIR --allow-run=cargo $BASE_DIR/test262.js $@
