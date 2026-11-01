#!/bin/sh
# Assemble the same Finder app for local installation and CI distribution.
set -eu
if [ "$(uname -s)" != Darwin ]; then
    echo 'macOS app bundling requires macOS.' >&2
    exit 1
fi
if [ "$#" -ne 2 ]; then
    echo 'Usage: scripts/bundle-macos.sh EXECUTABLE Cratebook.app' >&2
    exit 1
fi
binary=$1
bundle=$2
repo=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$repo/Cargo.toml" | head -1)
test -x "$binary"
mkdir -p "$bundle/Contents/MacOS" "$bundle/Contents/Resources/licenses"
install -m 755 "$binary" "$bundle/Contents/MacOS/cratebook"
cp "$repo/LICENSE" "$repo/THIRD_PARTY.md" "$bundle/Contents/Resources/"
cp "$repo/assets/OFL.txt" "$repo/assets/JetBrainsMono-OFL.txt" \
    "$repo/vendor/gpui-pre-web/LICENSE-APACHE" "$bundle/Contents/Resources/licenses/"
cp "$repo/assets/phosphor/LICENSE" "$bundle/Contents/Resources/licenses/Phosphor-MIT.txt"
cat > "$bundle/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleIdentifier</key><string>com.kahwee.cratebook</string>
<key>CFBundleName</key><string>Cratebook</string>
<key>CFBundleDisplayName</key><string>Cratebook</string>
<key>CFBundleExecutable</key><string>cratebook</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>$version</string>
<key>CFBundleVersion</key><string>$version</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>
PLIST
plutil -lint "$bundle/Contents/Info.plist"
codesign --force --sign - "$bundle"
codesign --verify --strict "$bundle"
printf 'Created %s (version %s)\n' "$bundle" "$version"
