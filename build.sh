#!/usr/bin/env bash
set -euo pipefail

BIN_DIR="${KON_BIN_DIR:-$HOME/.cargo/bin}"

cargo build --release
cp -f "target/release/kon" "$BIN_DIR/kon"
chmod +x "$BIN_DIR/kon"
echo "Installed kon to $BIN_DIR/kon"
