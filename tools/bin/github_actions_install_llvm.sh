set -eu

PROGNAME=$(basename $0)
BASE_DIR=$(cd $(dirname $0); pwd)

# TODO: update by `make update-deps`
LLVM_VERSION=19
LLVM_SH_URL='https://apt.llvm.org/llvm.sh'

COMPONENTS='core orcjit x86'
LINK_TYPE='--link-static'

help() {
  cat <<EOF >&2
Install LLVM into //vendor for CI workflows.

USAGE:
  $PROGNAME
  $PROGNAME -h | --help

DESCRIPTION:
  This script installs pre-built LLVM into //vendor before building in CI workflows.

  The pre-built LLVM is downloaded by using $LLVM_SH_URL.
EOF
  exit 0
}

while [ $# -gt 0 ]
do
  case "$1" in
    '-h' | '--help')
      help
      ;;
    *)
      break
      ;;
  esac
done

# See https://apt.llvm.org/#llvmsh.
sudo apt-get install -y lsb-release wget software-properties-common gnupg
wget -O - $LLVM_SH_URL >llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh $LLVM_VERSION

# Install packages needed for //libs/jsruntime/build.sh to work.
# Static libraries are needed.
sudo apt-get install -y libtinfo-dev libxml2-dev zlib1g-dev libzstd-dev

# //libs/jsruntime/build.sh uses //vendor/bin/llvm-config.
mkdir -p vendor/bin
ln -sf $(which llvm-config-$LLVM_VERSION) vendor/bin/llvm-config

# tests
vendor/bin/llvm-config --version
vendor/bin/llvm-config --assertion-mode
vendor/bin/llvm-config --build-mode
vendor/bin/llvm-config --host-target
vendor/bin/llvm-config --targets-built
vendor/bin/llvm-config $LINK_TYPE --cxxflags
vendor/bin/llvm-config $LINK_TYPE --libdir
vendor/bin/llvm-config $LINK_TYPE --libs $COMPONENTS
vendor/bin/llvm-config $LINK_TYPE --system-libs $COMPONENTS
