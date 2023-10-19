# Usage:
#   echo 'var x = 1' | sh validate.sh
#   cat path/to/script.js | sh validate.sh
#   curl https://host/script.js -sG | sh validate.sh

# NOTE: Initially, we used `jq` for sorting object properties, but we faced the following error:
#
#   $ curl https://cdnjs.cloudflare.com/ajax/libs/typescript/5.3.2/typescript.min.js -sG | \
#       sh ./tests/validate.sh
#   jq: parse error: Exceeds depth limit for parsing at line 1, column 10761262
#
# `jq` does not have any command-line options and environment variables to avoid this situation.
# So, we stop using `jq` in this script.

# TODO: BigInt support
#
# acorn has already supported BigInt, but it set a BigInt value to Literal.value.
# As a result, JSON.stringify() tries serializing BigInt values and fails.
# acorn should replace BigInt values with null before calling JSON.stringify().

set -eu

cleanup() {
  if [ -n $SRC ]
  then
    rm -f $SRC
  fi

  if [ -n $EXPECTED ]
  then
    rm -f $EXPECTED
  fi

  if [ -n $ACTUAL ]
  then
    rm -f $ACTUAL
  fi
}

# Build if needed.
cargo build -rqp bee-estree
BEE_ESTREE=$(realpath $(dirname $0)/../../../target/release/bee-estree)

deno run npm:acorn --compact --ecma2022 --module </dev/null >/dev/null
ACORN_START=$(date +%s%N)
deno run npm:acorn --compact --ecma2022 --module </dev/null >/dev/null
ACORN_END=$(date +%s%N)
ACORN_BASELINE=$(expr $ACORN_END - $ACORN_START)

$BEE_ESTREE --module </dev/null >/dev/null
BEE_ESTREE_START=$(date +%s%N)
$BEE_ESTREE --module </dev/null >/dev/null
BEE_ESTREE_END=$(date +%s%N)
BEE_ESTREE_BASELINE=$(expr $BEE_ESTREE_END - $BEE_ESTREE_START)

SRC=
EXPECTED=
ACTUAL=
trap 'cleanup' EXIT

SRC=$(mktemp -t bee-estree.validate.src.XXXXXX)
cat >$SRC
echo "SIZE: $(du -b $SRC | cut -f 1)" >&2

EXPECTED=$(mktemp -t bee-estree.validate.expected.XXXXXX)
ACORN_START=$(date +%s%N)
deno run npm:acorn --compact --ecma2022 --module <$SRC >$EXPECTED
ACORN_END=$(date +%s%N)
ACORN_ELAPSED=$(expr $ACORN_END - $ACORN_START)
ACORN_DELTA=$(expr $ACORN_ELAPSED - $ACORN_BASELINE)

ACTUAL=$(mktemp -t bee-estree.validate.actual.XXXXXX)
BEE_ESTREE_START=$(date +%s%N)
$BEE_ESTREE --module <$SRC >$ACTUAL
BEE_ESTREE_END=$(date +%s%N)
BEE_ESTREE_ELAPSED=$(expr $BEE_ESTREE_END - $BEE_ESTREE_START)
BEE_ESTREE_DELTA=$(expr $BEE_ESTREE_ELAPSED - $BEE_ESTREE_BASELINE)

cat <<EOF | column -t -s, >&2
PARSER,ELAPSED (ns),BASELINE (ns),DELTA (ns)
acorn,$ACORN_ELAPSED,$ACORN_BASELINE,$ACORN_DELTA
bee-estree,$BEE_ESTREE_ELAPSED,$BEE_ESTREE_BASELINE,$BEE_ESTREE_DELTA
EOF

cargo run -rqp bee-jsoncmp -- $ACTUAL $EXPECTED
