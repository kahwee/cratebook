#!/bin/sh
set -eu
if [ "$(uname -s)" != Darwin ]; then
    echo 'The Finder app installer requires macOS.' >&2
    exit 1
fi
cd "$(dirname "$0")/.."
target_dir=${CARGO_TARGET_DIR:-"$PWD/target"}
cargo build --locked --release --bin cratebook --target-dir "$target_dir"
bundle="$HOME/Applications/Cratebook.app"
sh scripts/bundle-macos.sh "$target_dir/release/cratebook" "$bundle"
