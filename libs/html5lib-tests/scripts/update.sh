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
# if [ -f "$DATA_DIR/commit_hash" ]
# then
#   if [ "$(cat $DATA_DIR/commit_hash)" = "$COMMIT_HASH" ]
#   then
#     echo 'Already up-to-date'
#     exit 0
#   fi
# fi

echo "Updating $DATA_DIR..."
rm -rf $DATA_DIR
mkdir -p $DATA_DIR
echo $COMMIT_HASH >$DATA_DIR/commit_hash

# Excluded test files
rm -f $TEMP_DIR/tokenizer/xmlViolation.test
rm -f $TEMP_DIR/tree-construction/search-element.dat
#rm -f $TEMP_DIR/tree-construction/webkit02.dat

mv $TEMP_DIR/tokenizer $DATA_DIR/
mv $TEMP_DIR/tree-construction $DATA_DIR/tree_construction
mv $TEMP_DIR/LICENSE $DATA_DIR/

for file in $(cd $DATA_DIR/tokenizer; ls -1 *.test)
do
  snake_case="$(echo $file | sed -e 's/\([A-Z]\)/_\L\1/g')"
  if [ "$file" != "$snake_case" ]
  then
    mv -f "$DATA_DIR/tokenizer/$file" "$DATA_DIR/tokenizer/$snake_case"
  fi
done

for file in $(cd $DATA_DIR/tree_construction; ls -1 *.dat)
do
  snake_case="$(echo $file | tr '-' '_')"
  if [ "$file" != "$snake_case" ]
  then
    mv -f "$DATA_DIR/tree_construction/$file" "$DATA_DIR/tree_construction/$snake_case"
  fi
done
# special cases
mv -f $DATA_DIR/tree_construction/tests_innerHTML_1.dat \
  $DATA_DIR/tree_construction/tests_inner_html_1.dat
