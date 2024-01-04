BASE_DIR=$(cd $(dirname $0); pwd)
# webkit.js consumes a lot of memory...
deno run -q \
  --allow-run=cargo \
  --v8-flags='--max-old-space-size=2000' \
  $BASE_DIR/validate.js $@
