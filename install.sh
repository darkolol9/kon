#!/usr/bin/env bash
set -euo pipefail

REPO="darkolol9/kon"
BIN_DIR="${KON_INSTALL_DIR:-/usr/local/bin}"

detect_arch() {
  local arch
  arch=$(uname -m)
  case "$arch" in
    x86_64|amd64) echo "x86_64" ;;
    aarch64|arm64) echo "aarch64" ;;
    *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;
  esac
}

detect_os() {
  local os
  os=$(uname -s)
  case "$os" in
    Linux) echo "unknown-linux-musl" ;;
    Darwin) echo "apple-darwin" ;;
    *) echo "Unsupported OS: $os" >&2; exit 1 ;;
  esac
}

main() {
  local arch target version url tmpdir
  arch=$(detect_arch)
  target=$(detect_os)
  target="${arch}-${target}"
  version="${1:-latest}"

  if [ "$version" = "latest" ]; then
    url="https://github.com/${REPO}/releases/latest/download/kon-${target}.tar.gz"
  else
    url="https://github.com/${REPO}/releases/download/${version}/kon-${target}.tar.gz"
  fi

  tmpdir=$(mktemp -d)
  cd "$tmpdir"

  echo "Downloading kon ${version} for ${target}..."
  curl -fsSL "$url" -o kon.tar.gz

  echo "Extracting..."
  tar xzf kon.tar.gz

  echo "Installing to ${BIN_DIR}/kon..."
  if [ ! -d "$BIN_DIR" ]; then
    mkdir -p "$BIN_DIR"
  fi
  install -m 755 kon "$BIN_DIR/kon"

  rm -rf "$tmpdir"
  echo "kon installed successfully!"
}

main "$@"
