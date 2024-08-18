set -eu

PROGNAME=$(basename $0)
BASEDIR=$(cd $(dirname $0); pwd)
PROJDIR=$(cd $BASEDIR/../..; pwd)
DOCKERFILE=.devcontainer/Dockerfile

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

IMAGE='mcr.microsoft.com/vscode/devcontainers/rust'
CLEAN=no

help() {
  cat <<EOF >&2
Update $DOCKERFILE.

USAGE:
  $PROGNAME [options]
  $PROGNAME -h | --help

OPTIONS:
  -h, --help
    Show this screen.

  -c, --clean
    Remove $IMAGE at exit.
EOF
  exit 0
}

clean() {
  sleep 1
  if [ "$CLEAN" = yes ]
  then
    $DOCKER image rm -f $IMAGE >/dev/null
    log "Removed $IMAGE"
  fi
  rm -f $TEMP_FILE
}

while [ $# -gt 0 ]
do
  case "$1" in
    '-h' | '--help')
      help
      ;;
    '-c' | '--clean')
      CLEAN=yes
      shift
      ;;
    *)
      break
      ;;
  esac
done

TEMP_FILE=$(mktemp)
trap "clean" EXIT INT TERM

log "Downloading $IMAGE..."
$DOCKER image pull $IMAGE

log "Getting the commit hash of rustc contained in $IMAGE..."
RUSTC_COMMIT_HASH=$($DOCKER run --rm $IMAGE rustc -vV | \
                      grep 'commit-hash' | cut -d ' ' -f 2)

log "Getting the path of the default toolchain contained in $IMAGE..."
RUST_TOOLCHAIN_PATH=$($DOCKER run --rm $IMAGE rustup toolchain list -v | \
                        grep '(default)' | cut -f 2)

cat <<EOF
--------------------------------------------------------------------------------
RUSTC_COMMIT_HASH  : $RUSTC_COMMIT_HASH
RUST_TOOLCHAIN_PATH: $RUST_TOOLCHAIN_PATH
--------------------------------------------------------------------------------
EOF

log "Updating sourcemap variables in $DOCKERFILE..."
# Don't use the -i option of `sed`.
# The incompatibility between macOS and GNU will cause troubles.
#
# Use `|` instead of `/` because RUST_TOOLCHAIN_PATH contains `/`.
sed -e "s|^ENV RUSTC_COMMIT_HASH=.*|ENV RUSTC_COMMIT_HASH=\"$RUSTC_COMMIT_HASH\"|" \
    -e "s|^ENV RUST_TOOLCHAIN_PATH=.*|ENV RUST_TOOLCHAIN_PATH=\"$RUST_TOOLCHAIN_PATH\"|" \
    $PROJDIR/$DOCKERFILE >$TEMP_FILE
mv -f $TEMP_FILE $PROJDIR/$DOCKERFILE

if git -C $PROJDIR diff --quiet -- $DOCKERFILE
then
  log "Not changed"
else
  log "Updated"
  git -C $PROJDIR add $DOCKERFILE
  git -C $PROJDIR commit -m "build: update $DOCKERFILE"
fi
