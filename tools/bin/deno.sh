OPTIONS="-q -A --unstable"
if [ -n "$BEE_TOOLS_DENO_OPTIONS" ]
then
  OPTIONS="$OPTIONS $BEE_TOOLS_DENO_OPTIONS"
fi
deno run $OPTIONS "$@"
