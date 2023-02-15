BASEDIR=$(cd $(dirname $0); pwd)
HTML5LIB_TESTS_GIT=https://github.com/html5lib/html5lib-tests.git
HTML5LIB_TESTS_DIR=$BASEDIR/data

TEMPDIR=$(mktemp -d)
trap "rm -rf $TEMPDIR" EXIT

echo "Cloning $HTML5LIB_TESTS_GIT..."
git clone -q --depth=1 $HTML5LIB_TESTS_GIT $TEMPDIR

echo "Updating $HTML5LIB_TESTS_DIR..."
rm -rf $HTML5LIB_TESTS_DIR
mkdir -p $HTML5LIB_TESTS_DIR
mv $TEMPDIR/tree-construction/* $HTML5LIB_TESTS_DIR/
mv $TEMPDIR/LICENSE $HTML5LIB_TESTS_DIR/
