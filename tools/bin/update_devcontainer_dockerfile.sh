set -eu

PROGNAME=$(basename $0)
BASEDIR=$(cd $(dirname $0); pwd)
PROJDIR=$(cd $BASEDIR/..; pwd)
TARGET_FILE=.devcontainer/Dockerfile

if [ "$(uname)" != Linux ] || id -nG | grep -q docker
then
  DOCKER='docker'
else
  DOCKER='sudo docker'
fi

IMAGE='mcr.microsoft.com/vscode/devcontainers/rust'
CLEAN=no

help() {
  cat <<EOF >&2
Update Dockerfile for VSCode Remote Container.

USAGE:
  $PROGNAME [options]
  $PROGNAME -h | --help

OPTIONS:
  -h, --help
    Show help.

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
    echo "Removed $IMAGE"
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

if [ "$(pwd)" != "$PROJDIR" ]
then
  echo "ERROR: must run in the project root"
  exit 1
fi

TEMP_FILE=$(mktemp)
trap "clean" EXIT INT TERM

echo "Downloading $IMAGE..."
$DOCKER image pull $IMAGE

echo "Getting the commit hash of rustc contained in $IMAGE..." >&2
RUSTC_COMMIT_HASH=$($DOCKER run --rm $IMAGE rustc -vV | \
                      grep 'commit-hash' | cut -d ' ' -f 2)

echo "Getting the path of the default toolchain contained in $IMAGE..." >&2
RUST_TOOLCHAIN_PATH=$($DOCKER run --rm $IMAGE rustup toolchain list -v | \
                        grep '(default)' | cut -f 2)

cat <<EOF
--------------------------------------------------------------------------------
RUSTC_COMMIT_HASH  : $RUSTC_COMMIT_HASH
RUST_TOOLCHAIN_PATH: $RUST_TOOLCHAIN_PATH
--------------------------------------------------------------------------------
EOF

echo "Updating sourcemap variables in $TARGET_FILE..." >&2
# Don't use the -i option of `sed`.
# The incompatibility between macOS and GNU will cause troubles.
#
# Use `|` instead of `/` because RUST_TOOLCHAIN_PATH contains `/`.
sed -e "s|^ENV RUSTC_COMMIT_HASH=.*|ENV RUSTC_COMMIT_HASH=\"$RUSTC_COMMIT_HASH\"|" \
    -e "s|^ENV RUST_TOOLCHAIN_PATH=.*|ENV RUST_TOOLCHAIN_PATH=\"$RUST_TOOLCHAIN_PATH\"|" \
    $TARGET_FILE >$TEMP_FILE
mv -f $TEMP_FILE $TARGET_FILE

if git diff --quiet -- $TARGET_FILE
then
  echo "Not changed"
else
  echo "Updated"
  git add $TARGET_FILE
  git commit -m "build: update $TARGET_FILE"
fi
