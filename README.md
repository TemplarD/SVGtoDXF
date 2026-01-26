# SVG to DXF Converter

Кроссплатформенное приложение для конвертации SVG файлов в DXF формат с современным интерфейсом.

## Возможности

- 🎨 **Конвертация SVG в DXF** - преобразование векторной графики в CAD формат
- 📁 **Массовая обработка** - выбор отдельных файлов или целых папок
- 📊 **Прогресс выполнения** - отслеживание процесса конвертации в реальном времени
- 🖥️ **Кроссплатформенность** - работает на Windows, Linux и macOS
- 🎯 **Простой интерфейс** - интуитивно понятный дизайн

## Архитектура

- **Frontend**: HTML/CSS/JavaScript с современным UI
- **Backend**: Rust + Tauri для нативной производительности
- **Библиотеки**: 
  - `dxf` - генерация DXF файлов
  - `rfd` - файловые диалоги
  - `walkdir` - обход файловой системы

## 🚀 Установка и запуск

### 📦 Готовые сборки (рекомендуется)

**Самый простой способ - использовать готовые сборки в папке `releases/`:**

#### Windows
- `releases/windows/svg-to-dxf-converter_0.1.0_x64-setup.exe` - установщик
- `releases/windows/app.exe` - портативная версия

#### Linux
- `releases/linux/svg-to-dxf-converter_0.1.0_amd64.AppImage` - универсальный формат
- `releases/linux/svg-to-dxf-converter_0.1.0_amd64.deb` - для Ubuntu/Debian
- `releases/linux/svg-to-dxf-converter-0.1.0-1.x86_64.rpm` - для Fedora/CentOS

Подробная инструкция по запуску есть в `releases/README.md`

### 🔧 Сборка из исходников

### Windows

#### Требования
- Windows 10/11
- [Rust](https://rustup.rs/) 1.70+
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) или Visual Studio 2019+

#### Установка
```powershell
# 1. Установка Rust (если еще не установлен)
winget install Rustlang.Rust.MSVC

# 2. Клонирование репозитория
git clone <repository-url>
cd svgtodxf

# 3. Сборка и запуск
cd src-tauri
cargo tauri dev
```

#### Создание EXE файла
```powershell
cd src-tauri
cargo tauri build
# EXE файл будет в src-tauri/target/release/bundle/msi/
```

### Ubuntu/Linux

#### Требования
- Ubuntu 20.04+ / Debian 11+
- Rust 1.70+
- Системные библиотеки для сборки

#### Установка зависимостей
```bash
# Обновление системы
sudo apt update && sudo apt upgrade -y

# Установка Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Установка зависимостей для Tauri
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### Сборка и запуск
```bash
# Клонирование репозитория
git clone <repository-url>
cd svgtodxf

# Сборка и запуск
cd src-tauri
cargo tauri dev
```

#### Создание DEB пакета
```bash
cd src-tauri
cargo tauri build
# DEB пакет будет в src-tauri/target/release/bundle/deb/
```

### macOS

#### Требования
- macOS 10.15+
- [Rust](https://rustup.rs/) 1.70+
- [Xcode Command Line Tools](https://developer.apple.com/xcode/)

#### Установка
```bash
# Установка Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Клонирование и сборка
git clone <repository-url>
cd svgtodxf/src-tauri
cargo tauri dev
```

## Использование

1. **Выбор файлов**: Нажмите "Выбрать файлы" для выбора отдельных SVG файлов
2. **Выбор папки**: Нажмите "Выбрать папку" для сканирования папки на наличие SVG файлов
3. **Конвертация**: Нажмите "Конвертировать" для начала процесса
4. **Мониторинг**: Следите за прогрессом в списке файлов

## Поддерживаемые элементы SVG

- ✅ Прямоугольники (`<rect>`)
- ✅ Окружности (`<circle>`)
- ✅ Линии (`<line>`)
- 🔄 Пути (`<path>`) - базовая поддержка

## Тестирование

```bash
# Запуск тестов
cd src-tauri
cargo test

# Запуск с выводом
cargo test -- --nocapture
```

## Структура проекта

```
svgtodxf/
├── README.md           # Основная документация
├── DOCS.md             # Техническая документация
├── .gitignore          # Правила Git
├── releases/           # 📦 Готовые сборки
│   ├── README.md       # Инструкция по запуску
│   ├── windows/        # Версии для Windows
│   │   ├── app.exe
│   │   └── svg-to-dxf-converter_0.1.0_x64-setup.exe
│   └── linux/          # Версии для Linux
│       ├── svg-to-dxf-converter_0.1.0_amd64.AppImage
│       ├── svg-to-dxf-converter_0.1.0_amd64.deb
│       └── svg-to-dxf-converter-0.1.0-1.x86_64.rpm
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── lib.rs      # Основная логика
│   │   ├── main.rs     # Точка входа
│   │   ├── commands/   # Tauri команды
│   │   ├── types/      # Типы данных
│   │   ├── utils/      # Утилиты
│   │   └── tests/      # Тесты
│   ├── Cargo.toml      # Зависимости Rust
│   └── tauri.conf.json # Конфигурация Tauri
├── dist/               # Frontend файлы
├── index.html          # Основной HTML
├── style.css           # Стили
└── script.js           # JavaScript логика
```

## Разработка

### Добавление новых элементов SVG

1. Расширьте функцию `convert_svg_to_dxf_simple` в `src-tauri/src/utils/converter.rs`
2. Добавьте парсинг новых элементов
3. Создайте соответствующие DXF примитивы

### Тестирование новых функций

```rust
#[test]
fn test_new_feature() {
    // Ваш тест
}
```

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

## Лицензия

MIT License

## Вклад в проект

1. Fork репозитория
2. Создайте feature ветку
3. Внесите изменения
4. Отправьте Pull Request

## Известные ограничения

- Поддержка кривых Безье упрощена
- Сложные трансформации SVG не поддерживаются
- Градиенты и текст не конвертируются

## Планы развития

- [ ] Улучшенная поддержка путей SVG
- [ ] Подработка текстовых элементов
- [ ] Поддержка градиентов
- [ ] Пакетная оптимизация
- [ ] CLI интерфейс
