# SVG to DXF Converter

> 🚀 Кроссплатформенный конвертер SVG → DXF на Rust + Yew + Tauri 2.0 (Linux · Windows · macOS)

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app)
[![Yew](https://img.shields.io/badge/Yew-0.21-green.svg)](https://yew.rs/)

## 📋 Обзор

**SVG to DXF Converter** — десктопное приложение для конвертации SVG‑файлов в DXF.
Особенности: цвета (ACI + true‑color), заливка линиями (hatch‑by‑lines), текст → MTEXT,
встроенные растры (PNG/JPEG/GIF → вектор), массовая конвертация, Drag & Drop.

Готовые сборки включены в репозиторий — `releases/`.

## 🏗️ Архитектура проекта

```
svg2dxf/
├── crates/                          # workspace
│   ├── core/                        # 🔧 Ядро конвертации (usvg → dxf)
│   │   ├── lib.rs                  # Публичный API (convert_svg_to_dxf[_with_options])
│   │   ├── converter.rs            # SvgConverter, ConversionOptions, тесселяция путей/текста
│   │   ├── raster.rs               # Трассировка <image> → вектор (marching squares)
│   │   └── error.rs                # Ошибки конвертации
│   ├── ui/                          # 🎨 Frontend на Yew (wasm)
│   │   ├── lib.rs                  # Точка входа Yew + разметка/страницы
│   │   ├── bindings.rs             # Вызовы Tauri из Yew (invoke)
│   │   ├── state/mod.rs            # FileItem, ConversionOptions, состояние файла/статуса
│   │   └── style.css               # Тёмная компактная тема
│   ├── tauri_app/                   # ⚙️ Tauri‑обёртка
│   │   ├── main.rs                 # Точка входа (tauri::Builder)
│   │   ├── lib.rs                  # AppState + tauri команды (convert_svg_to_dxf, convert_folder)
│   │   ├── api.rs                  # UI‑команды: диалог выбора файлов/папки, конвертация, статус/размер
│   │   └── tests.rs                # Юнит‑тесты tauri‑слоя
│   └── integration_tests/           # 🧪 Интеграционные тесты (TestResult)
├── build/                           # Скрипты сборки (collect_releases.sh, linux.sh, macos.sh, windows.ps1)
└── crates/tauri_app/                # (Tauri‑конфиг + build.rs + иконки внутри crates/tauri_app/)
```

## 🚀 Быстрый старт

### Требования
- **Rust** ≥ 1.77 (проверено на 1.93) + `cargo tauri` CLI 2.x (`cargo install tauri-cli`)
- **Node.js** ≥ 18 (проверено на 22) + **trunk** 0.21 (`npm install -g trunk`)
- Rust‑таргет **wasm32-unknown-unknown** (`rustup target add wasm32-unknown-unknown`)
- Системные зависимости Tauri (см. ниже)
- Для кросс‑сборки Windows: `rustup target add x86_64-pc-windows-gnu` + `mingw-w64`

> `Cargo.lock` **зафиксирован** — сборка воспроизводима.

```bash
git clone https://github.com/TemplarD/SVGtoDXF.git
cd SVGtoDXF
cargo install tauri-cli
npm install -g trunk
rustup target add wasm32-unknown-unknown    # wasm‑фронтенд
cargo tauri dev       # режим разработки
```

## 🎯 Возможности

### Конвертация
- ✅ **SVG‑элементы**: `path`, `rect`, `circle`, `line`, `polygon`, `polyline`
- ✅ **Текст** → DXF `MTEXT` (позиция, размер шрифта, семейство, цвет)
- ✅ **Цвета** `fill`/`stroke` → цвета DXF (ACI, ближайший к палитре). Опция **true‑color (группа 420)** пишется рядом — новые программы покажут точный оттенок, старые возьмут ACI (обратно совместимо)
- ✅ **Заливка линиями** (опция): замкнутые фигуры с `fill` заполняются параллельными линиями нужного цвета. *Настоящий HATCH в `dxf 0.5` недоступен — используется hatch‑by‑lines (LWPOLYLINE), совместим с любыми просмотрщиками.*
- ✅ **Растры** (PNG/JPEG/GIF, вложенные через `<image href>` или data‑URI) → трассируются в вектор через **marching squares** (точки чёрного/белого порога). *Чистый отдельный PNG/JPEG файла‑входом не поддерживается (фильтр выбора — только SVG).*
- ✅ Массовая конвертация, Drag & Drop, сохранение в выбранную папку, обработка ошибок

### Интерфейс
- 🎨 Тёмная тема, компактный адаптивный дизизайн (нет боковых полей, вписывается в любое окно)
- ⚙️ Панель настроек (tooltip на hover):
  - **Заливка линиями** — чередующие линии внутри фигур
  - **Сохранять цвета** — переносить цвета SVG (выкл → ч/б)
- 📋 Список файлов → статус (✓/✗) в реальном времени

## 🧪 Тестирование

```bash
cargo test --package svg2dxf-core          # core: 3 теста
cargo test --package svg2dxf-integration-tests  # интеграционные
cargo test --package svg2dxf-ui            # ui unit: 4 теста
cargo test --workspace                     # все
```

Реальные тесты:
- **core**: `test_converter_creation`, `test_layer_setting`, `test_simple_svg_conversion`
- **ui** (unit): `test_ui_module_compiles`, `test_yew_components`, `test_file_item_creation`, `test_file_status_equality`
- **integration_tests**: `test_test_result_creation`, `test_test_result_serialization`, `test_multiple_test_results`

## 🔧 Сборка

```bash
cargo tauri build                      # Linux
cargo tauri build --target x86_64-pc-windows-gnu      # Windows (MinGW, из Linux)
cargo tauri build --target x86_64-apple-darwin        # macOS Intel
```

### Кроссплатформенность

| ОС | Команда | Зависимости |
|----|---------|-------------|
| **Linux** (x86_64) | `cargo tauri build` | `libwebkit2gtk-4.1-dev`, `libssl-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev` |
| **Windows** (x86_64) | `cargo tauri build --target x86_64-pc-windows-gnu` | `mingw-w64`, `rustup target add x86_64-pc-windows-gnu`, `makensis` |
| **macOS** | `cargo tauri build --target x86_64-apple-darwin` | Xcode Command Line Tools |

Подробности: https://tauri.app/start/prerequisites

## 📦 Готовые сборки

Скомпилированные билды включены в репозиторий в `releases/` (скопированы из `target/` скриптом `build/collect_releases.sh`). Актуально для версии **1.0.0**.

### Linux (x86_64, проверено на Ubuntu 24.04)
| Формат | Путь | Установка |
|--------|------|-----------|
| **.deb** | `releases/linux/*.deb` | `sudo apt install ./releases/linux/"SVG to DXF Converter_1.0.0_amd64.deb"` |
| **.rpm** | `releases/linux/*.rpm` | `sudo dnf install ./releases/linux/SVG\ to\ DXF\ Converter-1.0.0-1.x86_64.rpm` |
| **AppImage** | `releases/linux/*.AppImage` | `chmod +x` и запуск |

### Windows (x86_64)
| Формат | Путь |
|--------|------|
| **NSIS‑installer** | `releases/windows/SVG to DXF Converter_1.0.0_x64-setup.exe` |

> Подпись отсутствует (её нельзя сделать вне Windows‑хоста) — SmartScreen предупреждение в порядке, «Подробнее → Всё равно запустить».

### Обновление билдов в репе
```bash
cargo tauri build --target x86_64-pc-windows-gnu   # при необходимости
bash build/collect_releases.sh
git add releases && git commit -m "build: обновить релизы"
```

## 🔍 Чем открывать DXF

- **LibreCAD** — просмотр/рефакторинг **DXF** (частично DWG)
- **Gwenview** — просмотр **SVG**/растров (KDE)
- `sudo apt install -y librecad gwenview`

## 📄 Лицензия

MIT — см. [LICENSE](LICENSE).

---

## 🗺️ План развития (future)

1. **Входные растры (PNG/JPEG/GIF как отдельный файл)** — сейчас поддерживаются только **вложенные** `<image href>` внутри SVG. Планируется:
   - расширить file‑filter в `crates/tauri_app/src/api.rs` (`.add_filter("Изображения", &["png","jpg","jpeg","gif"])`)
   - в `crates/core/src/converter.rs::convert_file` добавить ветку: если содержимое **не является SVG** (по сигнатуре), десериализовать его через `image crate` → передать в `raster::trace_image` напрямую (без usvg‑парсера)
2. **Гладкая трассировка** — сейчас marching squares даёт пиксельные LWPOLYLINE. Варианты улучшения:
   - `potrace` через C FFI (идеальные Bezier‑кривые, но тяжело собрать)
   - post‑processing: группировать соседние LWPOLYLINE → `dxf Path` с сплайнами (dxf 0.5 поддерживает Path)
3. **Android (Web / PWA)** — отдельная папка `web/` (JS+WASM) → PWA/TWA без адресной строки. Android SDK на диске — Windows‑бинды, из Linux нужен либо TWA‑обёртка, либо нативный APK (cargo‑apk / rust‑android‑gradle)
4. **Подпись Windows‑инсталлятора** — на Linux можно self‑signed через `osslsigncode` (SmartScreen всё равно ругнется), либо реальный сертификат на Windows‑хосте

**v2.0.0** — полная реструктуризация: модульная архитектура (core/ui/tauri_app), Yew + Tauri 2.0, true‑color + заливка линиями, трассировка растров.
