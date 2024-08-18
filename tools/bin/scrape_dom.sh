set -eu

PROGNAME=$(basename $0)
BASE_DIR=$(cd $(dirname $0); pwd)
PUPPETEER_DIR=$(realpath $BASE_DIR/../puppeteer)
SCRIPTS_DIR=$(realpath $PUPPETEER_DIR/scrape_dom)

LOGGING=0
VIEWPORT_WIDTH=1280
VIEWPORT_HEIGHT=720
USE_CDP=0
URL_OR_FILE=

help() {
  cat <<EOF >&2
Scrape a DOM tree and resources from a web page using a Puppeteer Docker container.

USAGE:
  $PROGNAME [options] [<url-or-file>]
  $PROGNAME -h | --help

OPTIONS:
  --logging
    Enable logging.

  --viewport=<size> [default: ${VIEWPORT_WIDTH}x${VIEWPORT_HEIGHT}]
    A size of the viewport where the content is rendered.

ARGUMENTS:
  <url-or-file>
    An URL or a path to an existing file to load.

    Read an HTML content from STDIN if the <url-or-file> is not specified.
EOF
  exit 0
}

log() {
  echo "$1" >&2
}

error() {
  log "ERROR: $1"
  exit 1
}

while [ $# -gt 0 ]
do
  case "$1" in
    '-h' | '--help')
      help
      ;;
    '--logging')
      LOGGING=1
      shift
      ;;
    '--viewport')
      VIEWPORT_WIDTH=$(echo "$2" | cut -dx -f1)
      VIEWPORT_HEIGHT=$(echo "$2" | cut -dx -f2)
      shift 2
      ;;
    '--cdp')
      # Use Chrome devtools protocol for scraping (experimental).
      # FIXME: this option does not work properly...
      USE_CDP=1
      shift
      ;;
    *)
      URL_OR_FILE="$1"
      shift
      break
      ;;
  esac
done

if [ -z "$URL_OR_FILE" ]
then
  trap 'rm -f $URL_OR_FILE' EXIT
  URL_OR_FILE=$(mktemp scrape_dom_XXXXXXXX.html)
  cat >"$URL_OR_FILE"
  # Allow any user to read.
  chmod +r "$URL_OR_FILE"
fi

MOUNT_OPTIONS=$(cat <<EOF | tr '\n' ' '
-v $SCRIPTS_DIR/take_snapshot.js:/scrape_dom/take_snapshot.js:ro
-v $SCRIPTS_DIR/transfer_data.js:/scrape_dom/transfer_data.js:ro
EOF
)

if echo "$URL_OR_FILE" | grep -E '^(https?|file|data):' 1>/dev/null 2>/dev/null
then
  # URL
  PAGE_URL="$URL_OR_FILE"
else
  if [ -f "$URL_OR_FILE" ]
  then
    # File
    FILENAME=$(basename "$URL_OR_FILE")
    PAGE_URL="file:///scrape_dom/$FILENAME"
    MOUNT_OPTIONS="$MOUNT_OPTIONS -v $(realpath $URL_OR_FILE):/scrape_dom/$FILENAME:ro"
  else
    error "<url-or-file> must be an URL or a path to an existing file"
  fi
fi

sh $BASE_DIR/puppeteer_run.sh $PUPPETEER_DIR/scrape_dom.js -- \
  -e PAGE_URL="$PAGE_URL" \
  -e LOGGING=$LOGGING \
  -e VIEWPORT_WIDTH=$VIEWPORT_WIDTH \
  -e VIEWPORT_HEIGHT=$VIEWPORT_HEIGHT \
  -e USE_CDP=$USE_CDP \
  $MOUNT_OPTIONS
