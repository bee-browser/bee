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
  $PROGNAME [options] <install-dir>
  $PROGNAME -h | --help

OPTIONS:
  -c, --clean
    Remove the 'node' image.

ARGUMENTS:
  <install-dir>
    The path to a folder where 'v8' and 'v8.d' will be installed.

DESCRIPTION:
  This script installs the 'v8' command and the 'v8.d' folder into <install-dir>/bin/.

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

INSTALL_DIR=$(realpath "$1")
if [ -z "$INSTALL_DIR" ]
then
  error "<install-dir> is required"
fi

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

rm -fr $INSTALL_DIR/bin/v8.d

SCRIPT="npx -y jsvu --os=$OS --engines=v8"
SCRIPT="$SCRIPT && cp -f -R -v /root/.jsvu/engines/v8 /outdir/v8.d"
SCRIPT="$SCRIPT && chown $(id -u):$(id -g) /outdir/v8.d"

mkdir -p $INSTALL_DIR/bin
$DOCKER run --rm -t --mount type=bind,source="$INSTALL_DIR/bin",target=/outdir node bash -c "$SCRIPT"

cat <<EOF >$INSTALL_DIR/bin/v8
#!/bin/sh
exec $INSTALL_DIR/bin/v8.d/v8 --snapshot_blob="$INSTALL_DIR/bin/v8.d/snapshot_blob.bin" "\$@"
EOF

chmod +x $INSTALL_DIR/bin/v8

# tests
test $($INSTALL_DIR/bin/v8 -e 'print(0)' | grep '0')
