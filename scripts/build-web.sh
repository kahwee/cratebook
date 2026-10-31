#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
export PATH="$PWD/.tools:$PATH"
target_dir=${CARGO_TARGET_DIR:-target}
if ! command -v wasm-bindgen >/dev/null; then
    echo "Install wasm-bindgen-cli 0.2.130 first (see README)." >&2
    exit 1
fi
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS='--cfg=web_sys_unstable_apis -C link-arg=-zstack-size=8388608'
cargo build --locked --target-dir "$target_dir" --target wasm32-unknown-unknown --lib --release
wasm-bindgen --target web --out-dir web/pkg "$target_dir/wasm32-unknown-unknown/release/cratebook.wasm"
