export RUST_LOG=none

OUTPUT="cdnjs.$(date +%Y%m%d%H%M).ndjson"

touch $OUTPUT

FILTER='.results[] | select(.latest != null) | select(.fileType == "js")'

TOTAL=$(curl https://api.cdnjs.com/libraries?fields=fileType -sG | jq -c "$FILTER" | wc -l)

COUNT=1
FAILED=0
BROKEN_LINKS=0

for LIB in $(curl https://api.cdnjs.com/libraries?fields=fileType -sG | jq -c "$FILTER")
do
  URL=$(echo "$LIB" | jq -r '.latest')
  if ! curl "$URL" -fsI >/dev/null
  then
    echo "$COUNT/$TOTAL: ! $URL"
    BROKEN_LINKS=$(expr $BROKEN_LINKS + 1)
  else
    METRICS=$(curl "$URL" -sG | cargo run -rq --example=jsparser -- --module 2>/dev/null)
    if [ "$?" = 0 ]
    then
      echo "$COUNT/$TOTAL: + $URL"
      TIME=$(echo "$METRICS" | cut -d ' ' -f 1 | cut -d '=' -f 2)
      SIZE=$(echo "$METRICS" | cut -d ' ' -f 2 | cut -d '=' -f 2)
      MAX_STACK_DEPTH=$(echo "$METRICS" | cut -d ' ' -f 3 | cut -d '=' -f 2)
      MAX_TEMPLATE_LITERAL_DEPTH=$(echo "$METRICS" | cut -d ' ' -f 4 | cut -d '=' -f 2)
      RESULT=$(cat <<EOF | tr -d '\n'
{
"url":"$URL",
"parsed":true,
"time":$TIME,
"size":$SIZE,
"maxStackDepth":$MAX_STACK_DEPTH,
"maxTemplateLiteralDepth":$MAX_TEMPLATE_LITERAL_DEPTH
}
EOF
)
      echo "$RESULT" >>$OUTPUT
    else
      echo "$COUNT/$TOTAL: - $URL"
      FAILED=$(expr $FAILED + 1)
      cat <<EOF >>$OUTPUT
{"url":"$URL","parsed":false}
EOF
    fi
  fi
  COUNT=$(expr $COUNT + 1)
done

PASSED=$(expr $TOTAL - $FAILED - $BROKEN_LINKS)

cat <<EOF
$TOTAL urls: $PASSED parsed, $FAILED failed, $BROKEN_LINKS broken links
EOF
