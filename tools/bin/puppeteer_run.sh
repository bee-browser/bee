set -eu

PROGNAME="$(basename $0)"

# TODO: Update by `make update-deps`
PUPPETEER_IMAGE_VERSION=23.1.0
DOCKER_IMAGE="ghcr.io/puppeteer/puppeteer:$PUPPETEER_IMAGE_VERSION"

if [ "$(uname)" != Linux ] || id -nG | grep -q docker; then
  DOCKER='docker'
else
  DOCKER='sudo docker'
fi

SCRIPT=
EXTRA_OPTIOS=

help() {
  cat <<EOF >&2
Evaluate JavaScript code in a Puppeteer Docker container.

USAGE:
  $PROGNAME [<script>] -- [<docker-run-options>...]
  $PROGNAME -h | --help

ARGUMENTS:
  <script>
    JavaScript code or a path to an existing JavaScript file.

    Read JavaScript code from STDIN if the <script> is not specified.

DESCRIPTION:
  The JavaScript code must be a CommonJS module.  It will be evaluated by
  Node.js inside the Docker container created from the Docker image
  $DOCKER_IMAGE.

EXAMPLES:
  Evaluate JavaScript code:
    $PROGNAME "console.log('hi there')"

  Evaluate JavaScript code in a file:
    $PROGNAME path/to/script.js

  Evaluate JavaScript code read from STDIN:
    echo "console.log('hi there')" | $PROGNAME

  Pass parameters to JavaScript code:
    $PROGNAME 'console.log(process.env.PARAM)' -- -e PARAM=1
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
    '--')
      shift
      break
      ;;
    *)
      if [ -z "$SCRIPT" ]
      then
        SCRIPT="$1"
      fi
      shift
      ;;
  esac
done

if [ -z "$SCRIPT" ]
then
  SCRIPT="$(cat)"
fi

if [ -f "$SCRIPT" ]
then
  SCRIPT="$(cat $SCRIPT)"
fi

$DOCKER run -i --init --cap-add=SYS_ADMIN --rm $* $DOCKER_IMAGE \
  node -e "$SCRIPT"
