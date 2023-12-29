BASE_DIR=$(cd $(dirname $0); pwd)
deno run -q --allow-read --allow-run=cargo $BASE_DIR/test262_parser_tests.js $@
