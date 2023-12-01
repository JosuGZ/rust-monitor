set -o errexit
set -o nounset
set -o pipefail

DEST_DIR=$DEST_DIR

BIN=rust-monitor
BINNAME=rust-monitor
PROFILE=release
cargo build --$PROFILE
cp target/$PROFILE/$BINNAME $DEST_DIR/Linux
