set -eu

PROGNAME=$(basename $0)
BASEDIR=$(dirname $0)
PROJDIR=$(cd $BASEDIR/../..; pwd)

# `bytehound` uses `yarn` for building the web UI.
# However, we don't use `yarn` (and node.js) in development.
# Instead, we use a `node:slim` Docker container for building `bytehound`.
# And we extract binaries from the container.
IMAGE=bee/bytehound

if id -nG | grep -q docker
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
Install 'bytehound'.

USAGE:
  $PROGNAME [options] <install-dir>
  $PROGNAME -h | --help

OPTIONS:
  -c, --clean
    Remove the $IMAGE image.

ARGUMENTS:
  <install-dir>
    The path to a folder where 'bytehound' and 'libbytehound.so' will be installed.

DESCRIPTION:
  This script installs 'bytehound' and 'libbytehound.so' into <install-dir>/bin/ and
  <install-dir>/lib/ respectively.

  Those will be built in a Docker container and extracted the binaries from it.
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

CONTAINER=
WORKDIR=

cleanup() {
  if [ -n "$CONTAINER" ]
  then
    $DOCKER container rm -f "$CONTAINER" >/dev/null
  fi
  if [ -n "$WORKDIR" ]
  then
    rm -fr "$WORKDIR"
  fi
  if [ -n "$CLEAN" ]
  then
    $DOCKER image rm -f $IMAGE >/dev/null
    log "Removed $IMAGE"
  fi
}

trap "cleanup" EXIT

WORKDIR=$(mktemp -d --suffix=_bee_bytehound)

cat <<'EOF' >$WORKDIR/Dockerfile
FROM node:slim AS build
COPY build.sh /
RUN sh -x /build.sh

FROM debian:stable-slim
COPY --from=build /bytehound/target/release/libbytehound.so /
COPY --from=build /bytehound/target/release/bytehound /

ENTRYPOINT ["/bytehound"]
CMD []
EOF

cat <<'EOF' >$WORKDIR/build.sh
set -eu

export DEBIAN_FRONTEND=noninteractive

GIT_URL=https://github.com/koute/bytehound.git

apt-get update
apt-get install -y build-essential curl git

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

git clone --recursive --depth=1 $GIT_URL
(cd bytehound && \
 cargo build --release -p bytehound-preload && \
 cargo build --release -p bytehound-cli)
EOF

$DOCKER build -t $IMAGE $WORKDIR

# test
$DOCKER run --rm $IMAGE --version | head -1 | grep bytehound-cli >/dev/null

CONTAINER=$($DOCKER create $IMAGE)
mkdir -p $INSTALL_DIR/bin
$DOCKER cp $CONTAINER:/bytehound $INSTALL_DIR/bin/bytehound
mkdir -p $INSTALL_DIR/lib
$DOCKER cp $CONTAINER:/libbytehound.so $INSTALL_DIR/lib/libbytehound.so

# test
$INSTALL_DIR/bin/bytehound --version head -1 | grep bytehound-cli >/dev/null

cat <<EOF >$INSTALL_DIR/bin/bytehound_serve
#!/bin/sh
echo 'Perfoming the command for the heap profiling...'
LD_PRELOAD=$INSTALL_DIR/lib/libbytehound.so \$@
echo 'Starting the bytehound server on http://localhost:8080...'
$INSTALL_DIR/bin/bytehound server memory-profiling_*.dat
EOF

chmod +x $INSTALL_DIR/bin/bytehound_serve
