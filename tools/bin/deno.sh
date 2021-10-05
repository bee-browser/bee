TOOLS_DIR=$(cd $(dirname $0)/..; pwd)
OPTIONS="-q -A --unstable --import-map=$TOOLS_DIR/import-map.json"
if [ -n "$BEE_TOOLS_DENO_OPTIONS" ]
then
  OPTIONS="$OPTIONS $BEE_TOOLS_DENO_OPTIONS"
fi
deno run $OPTIONS "$@"
