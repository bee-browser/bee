BASE_DIR=$(cd $(dirname $0); pwd)
deno run --allow-read --allow-run=cargo $BASE_DIR/test262-parser-tests.js $@
