set -o errexit
set -o nounset
set -o pipefail

BIN=rust-monitor
BINNAME=rust-monitor
PROFILE=release
cargo build --$PROFILE
cp target/$PROFILE/$BINNAME ~/Dropbox/Apps/Linux
