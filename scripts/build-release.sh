#!/bin/bash
set -e

PLATFORM=${1:-linux}
VERSION=$(node -p "require('./package.json').version")
OUT="$(pwd)/dist-release"
RELEASE_DIR="$(pwd)/src-tauri/target/release"

echo "building for $PLATFORM v$VERSION..."

rm -rf src-tauri/UTMT
if [ "$PLATFORM" = "windows" ]; then
    cp -r src-tauri/UTMT-windows src-tauri/UTMT
else
    cp -r src-tauri/UTMT-linux src-tauri/UTMT
fi

npm run tauri build -- --no-bundle

STAGE="$(mktemp -d)"
trap "rm -rf $STAGE" EXIT

if [ "$PLATFORM" = "windows" ]; then
    cp "$RELEASE_DIR/kd-music-tool.exe" "$STAGE/"
else
    cp "$RELEASE_DIR/kd-music-tool" "$STAGE/"
fi
cp -r src-tauri/UTMT "$STAGE/UTMT"

rm -rf "$OUT"
mkdir -p "$OUT"
ZIP="$OUT/kd-music-tool-v${VERSION}-${PLATFORM}.zip"
cd "$STAGE"
zip -r "$ZIP" .

echo "done: $ZIP ($(du -sh "$ZIP" | cut -f1))"
