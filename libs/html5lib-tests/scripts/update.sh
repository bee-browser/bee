set -eu

BASE_DIR=$(cd $(dirname $0); pwd)
PROJ_DIR=$(cd $BASE_DIR/..; pwd)
DATA_DIR=$PROJ_DIR/data
GIT_URL=https://github.com/html5lib/html5lib-tests.git

TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo "Cloning $GIT_URL..."
git clone -q --depth=1 $GIT_URL $TEMP_DIR

COMMIT_HASH="$(cd $TEMP_DIR; git show --format='%H' --no-patch)"
if [ -f "$DATA_DIR/commit_hash" ]
then
  if [ "$(cat $DATA_DIR/commit_hash)" = "$COMMIT_HASH" ]
  then
    echo 'Already up-to-date'
    exit 0
  fi
fi

echo "Updating $DATA_DIR..."
rm -rf $DATA_DIR
mkdir -p $DATA_DIR
echo $COMMIT_HASH >$DATA_DIR/commit_hash

mv $TEMP_DIR/tokenizer $DATA_DIR/
mv $TEMP_DIR/tree-construction $DATA_DIR/tree_construction
mv $TEMP_DIR/LICENSE $DATA_DIR/

rm -f $DATA_DIR/tokenizer/xmlViolation.test  # excluded
for file in $DATA_DIR/tokenizer/*.test
do
  snake_case="$(echo $file | sed -e 's/\([A-Z]\)/_\L\1/g')"
  if [ "$file" != "$snake_case" ]
  then
    mv -f "$file" "$snake_case"
  fi
done
