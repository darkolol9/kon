#!/usr/bin/env bash
set -euo pipefail

BIN_DIR="${KON_BIN_DIR:-$HOME/.cargo/bin}"

cargo build --release

# Atomic swap: temp file + mv works even when kon is running
TMP="$BIN_DIR/kon.tmp"
cp -f "target/release/kon" "$TMP"
chmod +x "$TMP"
mv -f "$TMP" "$BIN_DIR/kon"
echo "Installed kon to $BIN_DIR/kon"
