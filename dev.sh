#!/usr/bin/env bash
set -euo pipefail

BIN_DIR="${KON_BIN_DIR:-$HOME/.cargo/bin}"

if [[ "${1:-}" == "--watch" ]]; then
  if ! command -v cargo-watch &>/dev/null; then
    echo "cargo-watch not found. Install with: cargo install cargo-watch"
    exit 1
  fi
  exec cargo watch -w src/ -s "./dev.sh"
fi

cargo build

TMP="$BIN_DIR/kon.tmp"
cp -f "target/debug/kon" "$TMP"
chmod +x "$TMP"
mv -f "$TMP" "$BIN_DIR/kon"
echo "Installed kon (debug) to $BIN_DIR/kon"
