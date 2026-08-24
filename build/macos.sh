#!/usr/bin/env bash
# Сборка SVGtoDXF для macOS (Intel x86_64 и Apple Silicon aarch64)
# Требования:
#   xcode-select --install
#   rustup target add x86_64-apple-darwin aarch64-apple-darwin
#   npm install -g trunk
set -euo pipefail
cd "$(dirname "$0")/.."
echo "==> Сборка SVGtoDXF :: macOS (универсальный бандл)"
# Intel
cargo tauri build --target x86_64-apple-darwin
# Apple Silicon
cargo tauri build --target aarch64-apple-darwin
echo "==> Готово. Бандлы в: target/*-apple-darwin/release/bundle/"
