BASE_DIR=$(cd $(dirname $0); pwd)
deno run --allow-run=cargo $BASE_DIR/validate.js $@
