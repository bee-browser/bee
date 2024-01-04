BASE_DIR=$(cd $(dirname $0); pwd)
# webkit.js consumes a lot of memory...
deno run -q \
  --allow-net=deno.land,api.cdnjs.com,cdnjs.cloudflare.com \
  --allow-run=cargo \
  --allow-read=$BASE_DIR \
  --v8-flags='--max-old-space-size=2000' \
  $BASE_DIR/cdnjs.js $@
