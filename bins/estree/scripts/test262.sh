BASE_DIR=$(cd $(dirname $0); pwd)
PROJ_DIR=$(cd $BASE_DIR/../../..; pwd)
TEST262_DIR=$PROJ_DIR/vendor/tc39/test262
deno run -q --allow-read --allow-run=cargo $BASE_DIR/test262.js $@
