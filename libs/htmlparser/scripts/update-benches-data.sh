set -eu

BASE_DIR=$(cd $(dirname $0); pwd)
PROJ_DIR=$(cd $BASE_DIR/..; pwd)
DATA_DIR=$PROJ_DIR/benches/data
GIT_URL=https://github.com/y21/rust-html-parser-benchmark.git

TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo "Cloning $GIT_URL..."
git clone -q --depth=1 $GIT_URL $TEMP_DIR

COMMIT_HASH="$(cd $TEMP_DIR; git show --format='%H' --no-patch)"

echo "Updating $DATA_DIR..."
rm -rf $DATA_DIR
mkdir -p $DATA_DIR
echo $COMMIT_HASH >$DATA_DIR/commit_hash

mv $TEMP_DIR/data/*.html $DATA_DIR/
