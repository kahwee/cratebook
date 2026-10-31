#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
target_dir=${CARGO_TARGET_DIR:-"$PWD/target"}
cargo build --locked --release --target-dir "$target_dir"
bundle="$HOME/Applications/Cratebook.app"
mkdir -p "$bundle/Contents/MacOS"
install -m 755 "$target_dir/release/cratebook" "$bundle/Contents/MacOS/cratebook"
cat >"$bundle/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0"><dict>
<key>CFBundleIdentifier</key><string>com.kahwee.cratebook</string>
<key>CFBundleName</key><string>Cratebook</string>
<key>CFBundleExecutable</key><string>cratebook</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleVersion</key><string>1</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>
PLIST
codesign --force --sign - "$bundle"
