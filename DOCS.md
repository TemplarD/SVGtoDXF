# Документация проекта SVG to DXF Converter

## Обзор

Проект представляет собой кроссплатформенное приложение для конвертации SVG файлов в DXF формат с использованием Rust + Tauri.

## Архитектура

### Файловая структура

```
src-tauri/src/
├── lib.rs                    # Главный модуль с функцией run()
├── main.rs                   # Точка входа приложения
├── commands/                 # Tauri команды
│   ├── mod.rs
│   ├── select_files.rs       # Выбор файлов
│   ├── select_folder.rs      # Выбор папки
│   └── convert_svg.rs        # Конвертация
├── types/                    # Типы данных
│   ├── mod.rs
│   ├── conversion_file.rs
│   ├── conversion_status.rs
│   └── conversion_progress.rs
├── utils/                    # Утилиты
│   ├── mod.rs
│   └── converter.rs          # Логика конвертации
└── tests/                    # Тесты
    ├── mod.rs
    ├── test_converter.rs
    └── test_types.rs
```

### Основные компоненты

#### 1. Модуль типов (`types/`)
- `ConversionFile` - информация о файле для конвертации
- `ConversionStatus` - статусы конвертации (Pending, Processing, Completed, Error)
- `ConversionProgress` - информация о прогрессе конвертации

#### 2. Модуль команд (`commands/`)
- `select_files()` - выбор отдельных SVG файлов через диалог
- `select_folder()` - выбор папки с поиском SVG файлов
- `convert_svg_to_dxf()` - основная команда конвертации

#### 3. Модуль утилит (`utils/`)
- `convert_single_file()` - конвертация одного файла
- `convert_svg_to_dxf_simple()` - простая конвертация SVG элементов
- `extract_attr_value()` - извлечение атрибутов из SVG

#### 4. Тесты (`tests/`)
- `test_extract_attr_value()` - тест извлечения атрибутов
- `test_convert_svg_to_dxf_simple()` - тест конвертации
- `test_conversion_file_creation()` - тест создания структур
- `test_conversion_progress()` - тест прогресса

## Поддерживаемые SVG элементы

### ✅ Реализовано
- **Прямоугольники** (`<rect>`) - конвертируются в полилинии DXF
- **Окружности** (`<circle>`) - конвертируются в окружности DXF
- **Линии** (`<line>`) - конвертируются в линии DXF

### 🔄 В планах
- **Пути** (`<path>`) - поддержка кривых Безье
- **Эллипсы** (`<ellipse>`)
- **Многоугольники** (`<polygon>`)
- **Текст** (`<text>`)

## API команды

### select_files()
```rust
async fn select_files() -> Result<Vec<String>, String>
```
Возвращает список выбранных SVG файлов.

### select_folder()
```rust
async fn select_folder() -> Result<Vec<String>, String>
```
Возвращает список SVG файлов в выбранной папке.

### convert_svg_to_dxf()
```rust
async fn convert_svg_to_dxf(
    files: Vec<String>,
    app_handle: tauri::AppHandle,
) -> Result<String, String>
```
Конвертирует список файлов в DXF формат.

## События

### conversion-progress
Отправляется при обновлении прогресса конвертации:
```json
{
  "current": 1,
  "total": 10,
  "current_file": "path/to/file.svg"
}
```

### file-status
Отправляется при изменении статуса файла:
```json
["path/to/file.svg", {"Completed": null}]
```

## Зависимости

### Основные
- `tauri` - фреймворк для десктопных приложений
- `serde` - сериализация/десериализация
- `dxf` - работа с DXF файлами
- `rfd` - файловые диалоги
- `walkdir` - обход файловой системы

### SVG обработка
- `usvg` - парсинг SVG (в планах)
- `svgtypes` - типы SVG (в планах)

## 🚀 Сборка и запуск

### Windows
```powershell
# Установка Rust
winget install Rustlang.Rust.MSVC

# Сборка и запуск
cd src-tauri
cargo tauri dev

# Создание EXE
cargo tauri build
```

### Ubuntu/Linux
```bash
# Установка зависимостей
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# Сборка и запуск
cd src-tauri
cargo tauri dev

# Создание DEB пакета
cargo tauri build
```

### macOS
```bash
# Установка Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Сборка и запуск
cd src-tauri
cargo tauri dev
```

### Тесты
```bash
cd src-tauri
cargo test --lib
```

## Особенности реализации

1. **Модульная архитектура** - код разделен по функциональным модулям
2. **Асинхронность** - все файловые операции асинхронные
3. **Прогресс** - отслеживание прогресса в реальном времени
4. **Обработка ошибок** - детальная обработка ошибок с информативными сообщениями

## Ограничения

1. **Упрощенный парсинг** - не поддерживает сложные SVG конструкции
2. **Без трансформаций** - SVG трансформации игнорируются
3. **Базовые примитивы** - только прямоугольники, окружности и линии
4. **Без стилей** - цвета, толщина линий и стили игнорируются

## Будущие улучшения

1. **Полноценный SVG парсер** с использованием `usvg`
2. **Поддержка кривых Безье** и сложных путей
3. **Стили и атрибуты** - цвета, толщина линий
4. **Оптимизация** - объединение примитивов
5. **CLI интерфейс** для консольного использования
6. **Пакетная обработка** с настройками

## Тестирование

Проект включает юнит-тесты для основных функций:

```bash
# Запуск всех тестов
cargo test --lib

# Запуск конкретного теста
cargo test --lib test_extract_attr_value

# Запуск с выводом
cargo test --lib -- --nocapture
```

## Логирование

В режиме разработки включено логирование уровня INFO:
```rust
tauri_plugin_log::Builder::default()
    .level(log::LevelFilter::Info)
    .build()
```

## Кроссплатформенность

Приложение поддерживает:
- **Windows** - с подсистемой windows (без консоли в релизе)
- **Linux** - нативная поддержка
- **macOS** - нативная поддержка

## Производительность

- **Асинхронная обработка** - не блокирует UI
- **Потоковая конвертация** - файлы обрабатываются последовательно
- **Минимальные зависимости** - быстрый запуск
- **Эффективный парсинг** - простые строковые операции

## 🔧 Устранение проблем

### Windows
- **Ошибка сборки**: Установите Visual Studio Build Tools
- **Ошибка запуска**: Проверьте наличие Visual C++ Redistributable

### Linux
- **Ошибка сборки**: Установите все зависимости из раздела выше
- **Ошибка GTK**: `sudo apt install libwebkit2gtk-4.0-dev`

### macOS
- **Ошибка сборки**: Установите Xcode Command Line Tools
- **Ошибка сертификата**: `xcode-select --install`
