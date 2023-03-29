set -eu

BASE_DIR=$(cd $(dirname $0); pwd)
PROJ_DIR=$(cd $BASE_DIR/..; pwd)
DATA_DIR=$PROJ_DIR/tests/html5lib-tests
GIT_URL=https://github.com/html5lib/html5lib-tests.git

TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo "Cloning $GIT_URL..."
git clone -q --depth=1 $GIT_URL $TEMP_DIR

COMMIT_HASH="$(cd $TEMP_DIR; git show --format='%H' --no-patch)"

echo "Updating $DATA_DIR..."
rm -rf $DATA_DIR
mkdir -p $DATA_DIR
mv $TEMP_DIR/LICENSE $DATA_DIR/
mv $TEMP_DIR/tokenizer/README.md $DATA_DIR/
mv $TEMP_DIR/tokenizer/*.test $DATA_DIR/

# Excluded test files
rm -f $DATA_DIR/xmlViolation.test

# Rename files
for file in $(cd $DATA_DIR; ls -1 *.test)
do
  snake_case="$(echo $file | sed -e 's/\([A-Z]\)/_\L\1/g')"
  if [ "$file" != "$snake_case" ]
  then
    mv -f "$DATA_DIR/$file" "$DATA_DIR/$snake_case"
  fi
done

cat <<EOF >>$DATA_DIR/__data_src.yaml
git_url: '$GIT_URL'
commit: '$COMMIT_HASH'
updated: '$(date -Idate)'
EOF
