# SVG to DXF Converter

> 🚀 **Современный кроссплатформенный конвертер SVG в DXF с модульной архитектурой на Rust**

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app)
[![Yew](https://img.shields.io/badge/Yew-0.21-green.svg)](https://yew.rs/)

## 📋 Обзор

**SVG to DXF Converter** - профессиональное приложение для конвертации SVG файлов в DXF формат, разработанное с учетом лучших практик и современных технологий. Проект полностью соответствует техническому заданию и построен на модульной архитектуре.

### ✨ Ключевые особенности

- 🏗️ **Модульная архитектура** - изолированные компоненты (core, ui, tauri_app, integration_tests)
- 🎨 **Современный UI** - фронтенд на Yew с анимациями и Drag & Drop
- 🔧 **Централизованное логирование** - система tracing с файловой ротацией
- 🐛 **Дебаг-панель F12** - отладка и тестирование в реальном времени
- 🧪 **Автотестирование** - полная система интеграционных тестов
- 🌐 **Кроссплатформенность** - Windows, macOS, Linux
- ⚡ **Высокая производительность** - нативная скорость Rust

## 🏗️ Архитектура проекта

```
svg2dxf/
├── crates/
│   ├── core/                    # 🔧 Ядро конвертации
│   │   ├── lib.rs              # Публичный API
│   │   ├── converter.rs        # Логика конвертации SVG→DXF
│   │   └── error.rs            # Обработка ошибок
│   ├── ui/                      # 🎨 Frontend на Yew
│   │   ├── lib.rs              # Точка входа Yew
│   │   ├── components/         # UI компоненты
│   │   ├── state/              # Управление состоянием
│   │   └── bindings.rs         # Связь с Tauri
│   ├── tauri_app/               # ⚙️ Tauri интеграция
│   │   ├── main.rs             # Запуск приложения
│   │   ├── commands.rs         # Tauri команды
│   │   ├── logging.rs          # Система логирования
│   │   └── debug.rs            # Дебаг функциональность
│   └── integration_tests/       # 🧪 Автотесты
│       ├── lib.rs              # API тестов
│       ├── test_runner.rs      # Запуск тестов
│       └── tests/              # Наборы тестов
├── Cargo.toml                  # Workspace конфигурация
├── CHANGELOG.md                # Журнал разработки
└── README.md                   # Этот файл
```

## 🚀 Быстрый старт

### Требования

- Rust 1.77.2+
- Node.js 18+ (для фронтенд сборки)
- Системные зависимости для Tauri

### Установка и запуск

```bash
# Клонирование репозитория
git clone https://github.com/TemplarD/SVGtoDXF.git
cd SVGtoDXF

# Установка зависимостей
cargo install tauri-cli
npm install -g trunk

# Запуск в режиме разработки
cargo tauri dev

# Сборка для продакшена
cargo tauri build
```

## 🎯 Основные возможности

### Конвертация файлов
- ✅ Поддержка основных SVG элементов: `path`, `rect`, `circle`, `line`, `polygon`, `polyline`
- ✅ **Текст** (`text`) конвертируется в DXF `MTEXT` с сохранением позиции, размера шрифта (масштаб) и семейства
- ✅ **Растровые изображения** (`image`, PNG/JPEG/GIF) — **трассируются в вектор** через marching squares (изолиния по яркости → набор LWPOLYLINE). Вложенный SVG в `<image>` пока не поддерживается.
- ✅ Массовая конвертация файлов
- ✅ Drag & Drop интерфейс
- ✅ Автоматическое сохранение рядом с исходными файлами
- ✅ Обработка ошибок с детальными сообщениями

### Пользовательский интерфейс
- 🎨 Современный дизайн с анимациями
- 📱 Адаптивный интерфейс
- 🔄 Прогресс конвертации в реальном времени
- 📋 Список файлов со статусами
- ❌ Встроенная справка

### Отладка и тестирование
- 🐛 **F12** - дебаг панель с логами
- 🧪 Автоматические тесты UI
- 📊 Отчеты о системе
- 🔍 Детальная диагностика

## 🛠️ Технологический стек

### Backend (Rust)
- **usvg** - парсинг SVG
- **dxf** - генерация DXF
- **tracing** - логирование
- **tokio** - асинхронность
- **tauri 2.0** - фреймворк

### Frontend (Yew)
- **yew 0.21** - React-подобный фреймворк
- **wasm-bindgen** - WebAssembly интеграция
- **web-sys** - DOM API
- **gloo** - утилиты для WASM

### Тестирование
- **wasm-bindgen-test** - UI тесты
- **Интеграционные тесты** - сквозное тестирование
- **Модульные тесты** - unit тестирование

## 📖 Использование

### Базовая конвертация

1. Запустите приложение
2. Нажмите "Выбрать файлы" или перетащите SVG файлы
3. Нажмите "Конвертировать"
4. DXF файлы появятся рядом с исходными

### Горячие клавиши

- **F12** - Переключить режим отладки
- **Ctrl+O** - Выбрать файлы
- **Ctrl+F** - Выбрать папку
- **Ctrl+R** - Начать конвертацию
- **Delete** - Очистить список

### Дебаг режим

Нажмите **F12** для открытия панели отладки:
- 📋 Просмотр логов в реальном времени
- 🧪 Запуск автотестов
- 📊 Создание отчетов
- 🔍 Диагностика системы

## 🧪 Тестирование

### Запуск всех тестов

```bash
# Запуск UI тестов
cargo test --package svg2dxf-integration-tests

# Запуск core тестов  
cargo test --package svg2dxf-core

# Запуск всех тестов workspace
cargo test --workspace
```

### Доступные тесты

- `test_main_window_open` - проверка открытия UI
- `test_select_files_button` - тест кнопки выбора
- `test_f12_debug_toggle` - тест дебаг панели
- `test_drag_drop_zone` - тест Drag & Drop
- `test_conversion_progress` - тест прогресса конвертации

## 📝 Разработка

### Добавление новых функций

1. **Core модуль** - логика конвертации в `crates/core/src/converter.rs`
2. **UI компоненты** - новые компоненты в `crates/ui/src/components/`
3. **Tauri команды** - API в `crates/tauri_app/src/api.rs`
4. **Тесты** - в `crates/integration_tests/src/tests/`

### Сборка

```bash
# Разработка
cargo tauri dev

# Продакшен
cargo tauri build

# Для конкретных платформ
cargo tauri build --target x86_64-pc-windows-msvc
cargo tauri build --target x86_64-apple-darwin
cargo tauri build --target x86_64-unknown-linux-gnu
```

### 🌐 Кроссплатформенность (Windows / Linux / macOS)

Проект собирается под все три основные ОС через Tauri 2.0 (нативный webview, не Electron).

| ОС | Команда сборки | Системные зависимости |
|----|----------------|----------------------|
| **Linux** (x86_64) | `cargo tauri build` | `libwebkit2gtk-4.1-dev`, `libssl-dev`, `libayatana-appindicator3-dev`, ` librsvg2-dev`, пакетный менеджер (на Ubuntu: `sudo apt install ...`) |
| **Windows** (x86_64) | `cargo tauri build --target x86_64-pc-windows-msvc` | Инструменты сборки MSVC (Visual Studio Build Tools), WebView2 (идёт в Windows 11 по умолчанию) |
| **macOS** (Intel/Apple Silicon) | `cargo tauri build --target x86_64-apple-darwin` (или `aarch64-apple-darwin`) | Xcode Command Line Tools (`xcode-select --install`) |

> Полный список зависимостей для конкретной ОС — см. официальную документацию Tauri 2: https://tauri.app/start/prerequisites/

## 📦 Готовые сборки

После `cargo tauri build` (или кросс-сборки) готовые билды лежат в `target/`:

### Linux (собрано на x86_64, проверено на Ubuntu 24.04)
| Формат | Путь | Установка |
|--------|------|-----------|
| **.deb** | `target/release/bundle/deb/SVG to DXF Converter_1.0.0_amd64.deb` | `sudo apt install ./target/release/bundle/deb/"SVG to DXF Converter_1.0.0_amd64.deb"` |
| **.rpm** | `target/release/bundle/rpm/SVG to DXF Converter-1.0.0-1.x86_64.rpm` | `sudo dnf install ./target/release/bundle/rpm/SVG\ to\ DXF\ Converter-1.0.0-1.x86_64.rpm` |
| **AppImage** | `target/release/bundle/appimage/SVG to DXF Converter_1.0.0_amd64.AppImage` | `chmod +x` и запуск |

Удаление старой версии (если ставили через .deb):
```bash
sudo apt remove svg-to-dxf-converter
```

### Windows (кросс-сборка из Linux через MinGW, `x86_64-pc-windows-gnu`)
| Формат | Путь |
|--------|------|
| **Установщик NSIS** | `target/x86_64-pc-windows-gnu/release/bundle/nsis/SVG to DXF Converter_1.0.0_x64-setup.exe` |
| Голый .exe | `target/x86_64-pc-windows-gnu/release/svg2dxf-tauri-app.exe` |

> Подпись отсутствует (её нельзя сделать вне Windows-хоста) — при запуске Windows может показать предупреждение SmartScreen. Это нормально, «Подробнее → Всё равно запустить».

### macOS
**Не собирается на Linux** (нет Xcode SDK). Собирать только на реальном Mac:
```bash
cargo tauri build --target x86_64-apple-darwin   # Intel
cargo tauri build --target aarch64-apple-darwin  # Apple Silicon
```
Результат: `.app` и `.dmg` в `target/<target>/release/bundle/`.

### GitHub Releases
Билды можно выложить в релизы репозитория: создайте Release на
https://github.com/TemplarD/SVGtoDXF/releases и загрузите файлы из таблиц выше.
Тогда пользователи смогут скачать их по прямым ссылкам без сборки из исходников.

## 🔍 Чем открывать результат (DXF / SVG / 3D)

Лёгкий набор для просмотра (Ubuntu/Debian):
```bash
sudo apt install -y librecad gwenview
```
- **LibreCAD** — просмотр и редактирование **DXF** (частично DWG), нативный CAD-просмотрщик.
- **Gwenview** — просмотрщик **SVG**, растровой графики и превью 3D-форматов (KDE, очень лёгкий).
- Для полноценного 3D: `sudo apt install -y meshlab` (меш) или `freecad` (тяжёлый CAD).

## 📄 Лицензия

MIT License - см. файл [LICENSE](LICENSE)

## 🤝 Вклад

Вклады приветствуются! Пожалуйста, создайте Issue для багов или Feature Request.

---

**Версия 2.0.0** - Полная реструктуризация по ТЗ с модульной архитектурой на Rust + Yew + Tauri 2.0
