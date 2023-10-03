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
    METRICS=$(curl "$URL" -sG | \
              cargo run -r -q --example=bee-jsparser-demo -- --module 2>/dev/null)
    if [ "$?" = 0 ]
    then
      echo "$COUNT/$TOTAL: + $URL"
      TIME=$(echo "$METRICS" | cut -d ' ' -f 1 | cut -d '=' -f 2)
      SIZE=$(echo "$METRICS" | cut -d ' ' -f 2 | cut -d '=' -f 2)
      MAX_STACK_DEPTH=$(echo "$METRICS" | cut -d ' ' -f 3 | cut -d '=' -f 2)
      MAX_TEMPLATE_LITERAL_DEPTH=$(echo "$METRICS" | cut -d ' ' -f 4 | cut -d '=' -f 2)
      cat <<EOF | tr -d '\n' >>$OUTPUT
{
"url":"$URL",
"parsed":true,
"time":$TIME,
"size":$SIZE,
"maxStackDepth":$MAX_STACK_DEPTH,
"maxTemplateLiteralDepth":$MAX_TEMPLATE_LITERAL_DEPTH
}
EOF
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

VALID_LINKS=$(expr $TOTAL - $BROKEN_LINKS)

cat <<EOF
FAILED: $FAILED/$VALID_LINKS ($(expr $FAILED \* 100 / $VALID_LINKS)%)
EOF
