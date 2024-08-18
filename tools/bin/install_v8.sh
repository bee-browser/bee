set -eu

PROGNAME=$(basename $0)
BASEDIR=$(cd $(dirname $0); pwd)
PROJDIR=$(cd $BASEDIR/../..; pwd)

if [ "$(uname)" != Linux ] || id -nG | grep -q docker
then
  DOCKER='docker'
else
  DOCKER='sudo docker'
fi

log() {
  echo "$1" >&2
}

error() {
  log "ERROR: $1"
  exit 1
}

CLEAN=

help() {
  cat <<EOF >&2
Install v8 (d8).

USAGE:
  $PROGNAME [options] <bindir>
  $PROGNAME -h | --help

OPTIONS:
  -c, --clean
    Remove the 'node' image.

ARGUMENTS:
  <bindir>
    A path to the folder where 'v8' and 'v8.d' will be installed.

DESCRIPTION:
  This script installs the 'v8' command and the 'v8.d' folder into the specified folder.

  For downloading a pre-built binaries, 'jsvu' is used inside a Docker container created from the
  'node' image.
EOF
  exit 0
}

while [ $# -gt 0 ]
do
  case "$1" in
    '-h' | '--help')
      help
      ;;
    '-c' | '--clean')
      CLEAN=1
      shift
      ;;
    *)
      break
      ;;
  esac
done

clean() {
  sleep 1
  if [ -n "$CLEAN" ]
  then
    $DOCKER image rm -f node >/dev/null
    log "Removed the image"
  fi
}

trap "clean" EXIT INT TERM

ARCH=$(docker version | grep OS/Arch | head -1 | tr -d ' ' | cut -d':' -f2)
case $ARCH in
  linux/amd64)
    OS=linux64
    ;;
  *)
    error "unsupported development environment: $ARCH"
esac

OUTDIR=$(realpath "$1")

rm -fr $OUTDIR/v8.d

SCRIPT="npx -y jsvu --os=$OS --engines=v8"
SCRIPT="$SCRIPT && cp -f -R -v /root/.jsvu/engines/v8 /outdir/v8.d"
SCRIPT="$SCRIPT && chown $(id -u):$(id -g) /outdir/v8.d"

$DOCKER run --rm -t --mount type=bind,source="$OUTDIR",target=/outdir node bash -c "$SCRIPT"

cat <<EOF >$OUTDIR/v8
#!/bin/sh
exec $OUTDIR/v8.d/v8 --snapshot_blob="$OUTDIR/v8.d/snapshot_blob.bin" "\$@"
EOF

chmod +x $OUTDIR/v8

# tests
test $($OUTDIR/v8 -e 'print(0)' | grep '0')
