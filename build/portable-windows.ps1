# Сборка ПОРТАТИВНОЙ версии SVG to DXF для Windows (без инсталлятора)
# Запуск:  .\build\portable-windows.ps1   (в PowerShell от обычного пользователя)
#
# Требования:
#   - Rust + target x86_64-pc-windows-gnu (rustup target add x86_64-pc-windows-gnu)
#   - MinGW-w64 (x86_64-w64-mingw32-gcc) в PATH
#   - Tauri CLI (cargo install tauri-cli) — либо запускать через cargo tauri build
#
# Результат: build/portable/SVGtoDXF-Portable-x64.zip
#   внутри: svg2dxf-tauri-app.exe + WebView2Loader.dll
# Пользователь просто распаковывает и запускает .exe — ничего не ставится.

$ErrorActionPreference = "Stop"
$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
$Target = "x86_64-pc-windows-gnu"
$AppExe = "svg2dxf-tauri-app.exe"
$OutDir = Join-Path $Root "build/portable"
$ZipName = "SVGtoDXF-Portable-x64.zip"

Write-Host "🔧 Проверяем rust-таргет $Target ..."
rustup target add $Target 2>$null

Write-Host "📦 Собираем release под Windows (GNU) ..."
cargo tauri build --target $Target

$Src = Join-Path $Root "target/$Target/release"
$Portable = Join-Path $OutDir "svg2dxf-portable"
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
Remove-Item -Recurse -Force $Portable -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $Portable | Out-Null

Write-Host "📁 Формируем портативную папку ..."
Copy-Item (Join-Path $Src $AppExe) $Portable
$WebViewDll = Join-Path $Src "WebView2Loader.dll"
if (Test-Path $WebViewDll) { Copy-Item $WebViewDll $Portable } else { Write-Host "⚠️ WebView2Loader.dll не найден — пропускаем" }

@"
SVG to DXF Converter - ПОРТАТИВНАЯ версия (без установки)
==========================================================
1. Распакуйте всю папку в удобное место (флешку, диск).
2. Запустите svg2dxf-tauri-app.exe
3. Папки вывода и выбора сохраняются рядом (portable-режим).

Требуется: Windows 10+ и WebView2 Runtime (обычно уже в системе).
Без цифровой подписи SmartScreen может ругаться - "Подробнее -> Всё равно запустить".
"@ | Out-File -Encoding utf8 (Join-Path $Portable "ПРОЧТИ.txt")

Write-Host "🗜️ Упаковываем в zip: $ZipName ..."
$ZipPath = Join-Path $OutDir $ZipName
if (Test-Path $ZipPath) { Remove-Item $ZipPath }
Compress-Archive -Path $Portable -DestinationPath $ZipPath -Force

Write-Host "✅ Готово: $ZipPath"
(Get-Item $ZipPath).Length / 1MB | ForEach-Object { "{0:N1} MB" -f $_ }
