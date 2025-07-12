BASE_DIR=$(cd $(dirname $0); pwd)
PROJ_DIR=$(cd $BASE_DIR/../../..; pwd)

# Disable creating core dump files in this script.
#
# 'bjs' crashes in many test cases and many core dump files will be created in the
# /var/lib/systemd/coredump folder.  We can remove them manually, but they are usually removed
# automatically.  However, the age argument is 2w:
#
#   $ cat /usr/lib/tmpfiles.d/systemd.conf | grep coredump
#   d /var/lib/systemd/coredump 0755 root root 2w
#
# It's too long.  Additionally, the size of /var/lib/systemd/coredump folder will be larger than
# 4KB.  Because many core dump files will be created.
#
# For avoiding these situations, we simply disable creating core dump files in this script.
ulimit -Sc 0

deno run -q \
  --allow-env \
  --allow-run \
  --allow-read=$PROJ_DIR \
  $BASE_DIR/test262.js $@
