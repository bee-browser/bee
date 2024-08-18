set -eu

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

ARCH=$(docker version | grep OS/Arch | head -1 | tr -d ' ' | cut -d':' -f2)
case $ARCH in
  linux/amd64)
    OS=linux64
    ;;
  *)
    error "unsupported development environment: $ARCH"
esac

OUTDIR=$(realpath "$1")

rm -fr $OUTDIR/v8.d

SCRIPT="npx -y jsvu --os=$OS --engines=v8"
SCRIPT="$SCRIPT && cp -f -R -v /root/.jsvu/engines/v8 /outdir/v8.d"
SCRIPT="$SCRIPT && chown $(id -u):$(id -g) /outdir/v8.d"

$DOCKER run --rm -t --mount type=bind,source="$OUTDIR",target=/outdir node bash -c "$SCRIPT"

cat <<EOF >$OUTDIR/v8
#!/bin/sh
exec $OUTDIR/v8.d/v8 --snapshot_blob="$OUTDIR/v8.d/snapshot_blob.bin" "\$@"
EOF

chmod +x $OUTDIR/v8

# tests
test $($OUTDIR/v8 -e 'print(0)' | grep '0')
