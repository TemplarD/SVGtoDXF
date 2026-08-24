#!/usr/bin/env bash
# Сборка SVGtoDXF для Linux (x86_64)
# Требования (Ubuntu/Debian):
#   sudo apt update
#   sudo apt install -y libwebkit2gtk-4.1-dev libssl-dev \
#       libayatana-appindicator3-dev librsvg2-dev pkg-config \
#       build-essential curl wget file
# Rust: rustup target add x86_64-unknown-linux-gnu
set -euo pipefail
cd "$(dirname "$0")/.."
echo "==> Сборка SVGtoDXF :: Linux (x86_64-unknown-linux-gnu)"
cargo tauri build --target x86_64-unknown-linux-gnu
echo "==> Готово. Бандлы в: target/x86_64-unknown-linux-gnu/release/bundle/"
