BASE_DIR=$(cd $(dirname $0); pwd)
deno run -q \
  --allow-net=deno.land,api.cdnjs.com,cdnjs.cloudflare.com \
  --allow-run=cargo \
  --allow-read=$BASE_DIR/cdnjs_worker.js,$BASE_DIR/test262_helper.js \
  $BASE_DIR/cdnjs.js $@
