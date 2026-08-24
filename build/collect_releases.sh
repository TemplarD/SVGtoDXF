#!/usr/bin/env bash
# Копирует свежесобранные билды из target/ в папку releases/ (для коммита в git).
# Запускать ПОСЛЕ `cargo tauri build` и кросс-сборки Windows.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REL="$ROOT/releases"
mkdir -p "$REL/linux" "$REL/windows"

echo "== Linux =="
LINUX="$ROOT/target/release/bundle"
for f in \
  "$LINUX/deb/SVG to DXF Converter_1.0.0_amd64.deb" \
  "$LINUX/rpm/SVG to DXF Converter-1.0.0-1.x86_64.rpm" \
  "$LINUX/appimage/SVG to DXF Converter_1.0.0_amd64.AppImage" ; do
  if [ -f "$f" ]; then cp -v "$f" "$REL/linux/"; else echo "  WARN: нет $f"; fi
done

echo "== Windows =="
WIN="$ROOT/target/x86_64-pc-windows-gnu/release/bundle/nsis/SVG to DXF Converter_1.0.0_x64-setup.exe"
if [ -f "$WIN" ]; then cp -v "$WIN" "$REL/windows/"; else echo "  WARN: нет $WIN"; fi

# Портативная версия (zip без установки)
PORTABLE="$ROOT/build/portable/SVGtoDXF-Portable-x64.zip"
if [ -f "$PORTABLE" ]; then cp -v "$PORTABLE" "$REL/windows/"; else echo "  WARN: нет $PORTABLE (запустите build/portable-windows.sh)"; fi

echo "Готово. Теперь: git add releases && git commit"
