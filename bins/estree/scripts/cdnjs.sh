BASE_DIR=$(cd $(dirname $0); pwd)
deno run -q \
  --allow-env \
  --allow-net=deno.land,api.cdnjs.com,cdnjs.cloudflare.com \
  --allow-run=cargo \
  --allow-read=$BASE_DIR \
  $BASE_DIR/cdnjs.js $@
