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

- **Rust** ≥ 1.77 (проверено на 1.93) + `cargo tauri` CLI 2.x (`cargo install tauri-cli`)
- **Node.js** ≥ 18 (проверено на 22) + **trunk** 0.21 (`npm install -g trunk`)
- Rust-таргет **wasm32-unknown-unknown** (`rustup target add wasm32-unknown-unknown`)
- Системные зависимости Tauri (см. ниже, раздел «Кроссплатформенность»)
- Для кросс-сборки Windows: `rustup target add x86_64-pc-windows-gnu` + MinGW-w64

> `Cargo.lock` **зафиксирован в репозитории** — сборка воспроизводима и не требует `cargo update`.

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
- ✅ **Текст** (`text`) конвертируется в DXF `MTEXT` с сохранением позиции, размера шрифта (масштаб), семейства и **цвета**
- ✅ **Цвета заливок и обводок** (SVG `fill`/`stroke`) конвертируются в цвета DXF (ближайший индекс палитры AutoCAD ACI). Опционально — **точный цвет (true-color, группа 420)** рядом с ACI: новые программы (LibreCAD, AutoCAD 2004+) покажут точный оттенок, старые проигнорируют 420 и возьмут ACI (обратно совместимо)
- ✅ **Заливка линиями** (опция «Заливка линиями»): замкнутые фигуры с `fill` заполняются параллельными линиями нужного цвета поверх контура (настоящий HATCH в dxf 0.5 недоступен — используется hatch-by-lines, совместимый со всеми просмотрщиками)
- ✅ **Растровые изображения** (`image`, PNG/JPEG/GIF) — **трассируются в вектор** через marching squares (изолиния по яркости → набор LWPOLYLINE). Вложенный SVG в `<image>` пока не поддерживается.
- ✅ Массовая конвертация файлов
- ✅ Drag & Drop интерфейс
- ✅ Сохранение в выбранную папку (отдельные DXF рядом с именами исходных файлов)
- ✅ Обработка ошибок с детальными сообщениями

### Пользовательский интерфейс
- 🎨 Тёмная тема, компактный адаптивный дизайн (работает и на узких экранах)
- ⚙️ Панель настроек конвертации с подсказками (tooltip при наведении):
  - **Заливка линиями** — рисовать заливку параллельными линиями внутри фигур
  - **Сохранять цвета** — переносить цвета SVG (выключите для ч/б)
  - *Фоново (без чекбокса):* true-color (группа 420), трассировка вложенных растров
- 📱 Адаптивный интерфейс
- 🔄 Прогресс конвертации в реальном времени
- 📋 Список файлов со статусами

### Отладка
- 🐛 **F12** — индикатор режима отладки (красный бадж в левом верхнем углу). Детальная диагностика и логи пока не реализованы.

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

- **F12** — переключить индикатор режима отладки (красный бадж в левом верхнем углу)

### Дебаг режим

F12 включает/выключает режим отладки — в режиме он отображается красным баджем `🔧 ДЕБУГ РЕЖИМ (F12)` в левом верхнем углу окна. Детальная диагностика и логи пока не реализованы.

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

- **core**: `test_converter_creation`, `test_layer_setting`, `test_simple_svg_conversion`
- **ui** (unit): `test_ui_module_compiles`, `test_yew_components`, `test_file_item_creation`, `test_file_status_equality`
- **integration_tests**: `test_test_result_creation`, `test_test_result_serialization`, `test_multiple_test_results`

> UI-интеграционные тесты в браузере (test_main_window_open и т.п.) пока **не реализованы** — в README перечислены как план на будущее.

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
cargo tauri build --target x86_64-unknown-linux-gnu   # Linux
cargo tauri build --target x86_64-pc-windows-gnu      # Windows (MinGW/cросс-сборка из Linux)
# cargo tauri build --target x86_64-pc-windows-msvc    # Windows (только на Windows + MSVC Build Tools)
cargo tauri build --target x86_64-apple-darwin        # macOS Intel
```

### 🌐 Кроссплатформенность (Windows / Linux / macOS)

Проект собирается под все три основные ОС через Tauri 2.0 (нативный webview, не Electron).

| ОС | Команда сборки | Системные зависимости |
|----|----------------|----------------------|
| **Linux** (x86_64) | `cargo tauri build` | `libwebkit2gtk-4.1-dev`, `libssl-dev`, `libayatana-appindicator3-dev`, ` librsvg2-dev`, пакетный менеджер (на Ubuntu: `sudo apt install ...`) |
| **Windows** (x86_64) | `cargo tauri build --target x86_64-pc-windows-gnu` (MinGW, кросс-сборка из Linux) | `mingw-w64` (`sudo apt install mingw-w64`), `rustup target add x86_64-pc-windows-gnu`, NSIS (`makensis` для .exe) |
| **macOS** (Intel/Apple Silicon) | `cargo tauri build --target x86_64-apple-darwin` (или `aarch64-apple-darwin`) | Xcode Command Line Tools (`xcode-select --install`) |

> Полный список зависимостей для конкретной ОС — см. официальную документацию Tauri 2: https://tauri.app/start/prerequisites/

## 📦 Готовые сборки

Скомпилированные билды **включены в репозиторий** в папку [`releases/`](releases/) (копия из `target/` после сборки). Их можно скачать прямо из дерева репозитория на GitHub без сборки из исходников.

> Билды собираются локально командой `cargo tauri build`, затем копируются в `releases/` скриптом `build/collect_releases.sh`. Актуально для версии **1.0.0**.

### Linux (x86_64, проверено на Ubuntu 24.04)
| Формат | Путь в репозитории | Установка |
|--------|------|-----------|
| **.deb** | `releases/linux/SVG to DXF Converter_1.0.0_amd64.deb` | `sudo apt install ./releases/linux/"SVG to DXF Converter_1.0.0_amd64.deb"` |
| **.rpm** | `releases/linux/SVG to DXF Converter-1.0.0-1.x86_64.rpm` | `sudo dnf install ./releases/linux/SVG\ to\ DXF\ Converter-1.0.0-1.x86_64.rpm` |
| **AppImage** | `releases/linux/SVG to DXF Converter_1.0.0_amd64.AppImage` | `chmod +x` и запуск |

Удаление старой версии (если ставили через .deb):
```bash
sudo apt remove svg-to-dxf-converter
```

### Windows (x86_64, кросс-сборка из Linux через MinGW `x86_64-pc-windows-gnu`)
| Формат | Путь в репозитории |
|--------|------|
| **Установщик NSIS** | `releases/windows/SVG to DXF Converter_1.0.0_x64-setup.exe` |

> Подпись отсутствует (её нельзя сделать вне Windows-хоста) — при запуске Windows может показать предупреждение SmartScreen. Это нормально, «Подробнее → Всё равно запустить».

### macOS
**Не собирается на Linux** (нет Xcode SDK). Собирать только на реальном Mac:
```bash
cargo tauri build --target x86_64-apple-darwin   # Intel
cargo tauri build --target aarch64-apple-darwin  # Apple Silicon
```
Результат: `.app` и `.dmg` в `target/<target>/release/bundle/`. При необходимости скопируйте в `releases/macos/` вручную.

### Обновление билдов в репозитории
После `cargo tauri build` (и кросс-сборки) выполните:
```bash
bash build/collect_releases.sh
git add releases && git commit -m "build: обновить релизы"
```


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
