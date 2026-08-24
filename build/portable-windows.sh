#!/usr/bin/env bash
# ============================================================
#  Сборка ПОРТАТИВНОЙ версии SVG to DXF для Windows (без установки)
#  Запуск:  bash build/portable-windows.sh
#  Требует: rustup target x86_64-pc-windows-gnu, x86_64-w64-mingw32-gcc (MinGW)
#           (ставится: apt install gcc-mingw-w64-x86-64)
#
#  Результат: build/portable/SVGtoDXF-Portable-x64.zip
#  Внутри: svg2dxf-tauri-app.exe + WebView2Loader.dll
#  Пользователь просто распаковывает и запускает .exe — ничего не ставится.
# ============================================================
set -euo pipefail
cd "$(dirname "$0")/.."

TARGET="x86_64-pc-windows-gnu"
APP_EXE="svg2dxf-tauri-app.exe"
OUT_DIR="build/portable"
ZIP_NAME="SVGtoDXF-Portable-x64.zip"

echo "🔧 Проверяем rust-таргет $TARGET ..."
rustup target add "$TARGET" 2>/dev/null || true

echo "📦 Собираем release под Windows (GNU) ..."
cargo tauri build --target "$TARGET"

SRC="target/$TARGET/release"
mkdir -p "$OUT_DIR"

echo "📁 Формируем портативную папку ..."
# Очищаем предыдущую сборку
rm -rf "$OUT_DIR/svg2dxf-portable"
mkdir -p "$OUT_DIR/svg2dxf-portable"

# Копируем само приложение и рантайм WebView2
cp "$SRC/$APP_EXE"             "$OUT_DIR/svg2dxf-portable/"
cp "$SRC/WebView2Loader.dll"   "$OUT_DIR/svg2dxf-portable/" 2>/dev/null || \
  echo "⚠️  WebView2Loader.dll не найден (возможно уже вшит) — пропускаем"

# Маленький README для переносимости
cat > "$OUT_DIR/svg2dxf-portable/ПРОЧТИ.txt" <<'EOF'
SVG to DXF Converter — ПОРТАТИВНАЯ версия (без установки)
==========================================================
1. Распакуйте всю папку в удобное место (флешку, диск).
2. Запустите svg2dxf-tauri-app.exe
3. Папки вывода и выбора сохраняются рядом (portable-режим).

Требуется: Windows 10+ и установленный WebView2 Runtime
(обычно уже есть в системе; при необходимости:
 https://developer.microsoft.com/ru-ru/microsoft-edge/webview2/)

Без цифровой подписи Windows SmartScreen может показать предупреждение —
«Подробнее → Всё равно запустить».
EOF

echo "🗜️  Упаковываем в zip: $OUT_DIR/$ZIP_NAME ..."
( cd "$OUT_DIR" && zip -r -q "$ZIP_NAME" "svg2dxf-portable" )

echo "✅ Готово: $OUT_DIR/$ZIP_NAME"
ls -lh "$OUT_DIR/$ZIP_NAME"
