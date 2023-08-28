export RUST_LOG=none

OUTPUT="cdnjs.$(date +%Y%m%d%H%M).ndjson"

touch $OUTPUT

FILTER='.results[] | select(.latest != null) | select(.fileType == "js")'

TOTAL=$(curl https://api.cdnjs.com/libraries?fields=fileType -sG | jq -c "$FILTER" | wc -l)

COUNT=1
for LIB in $(curl https://api.cdnjs.com/libraries?fields=fileType -sG | jq -c "$FILTER")
do
  URL=$(echo "$LIB" | jq -r '.latest')
  if ! curl "$URL" -sGf >/dev/null
  then
    echo "$COUNT/$TOTAL: ! $URL"
  else
    METRICS=$(curl "$URL" -sG | cargo run -r -q --example=bee-jsparser-demo 2>/dev/null)
    if [ "$?" = 0 ]
    then
      echo "$COUNT/$TOTAL: + $URL"
      TIME=$(echo "$METRICS" | cut -d ' ' -f 1 | cut -d '=' -f 2)
      SIZE=$(echo "$METRICS" | cut -d ' ' -f 2 | cut -d '=' -f 2)
      DEPTH=$(echo "$METRICS" | cut -d ' ' -f 3 | cut -d '=' -f 2)
      cat <<EOF >>$OUTPUT
{"url":"$URL","parsed":true,"time":$TIME,"size":$SIZE,"maxStackDepth":$DEPTH}
EOF
    else
      echo "$COUNT/$TOTAL: - $URL"
      cat <<EOF >>$OUTPUT
{"url":"$URL","parsed":false}
EOF
    fi
  fi
  COUNT=$(expr $COUNT + 1)
done
