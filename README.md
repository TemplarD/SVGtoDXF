# SVG to DXF Converter v1.0.0

Professional vector graphics conversion tool with advanced features.

## 🚀 Features

### Core Functionality
- **SVG to DXF Conversion** - High-quality vector graphics conversion
- **Batch Processing** - Convert multiple files simultaneously
- **Real-time Progress** - Track conversion progress with animations
- **Error Handling** - Comprehensive error reporting and recovery

### Advanced Features
- **Debug Mode (F12)** - Advanced debugging and testing tools
- **File System Management** - Intelligent file access and alternative paths
- **Logging System** - Detailed operation logging with daily rotation
- **Autotesting** - Comprehensive UI and functionality testing

### User Experience
- **Modern UI** - Beautiful animated interface with tooltips
- **Drag & Drop** - Intuitive file handling
- **Status Animations** - Visual feedback for all operations
- **Responsive Design** - Works on different screen sizes

## 📦 Installation

### Download Releases
1. Go to [Releases](https://github.com/TemplarD/SVGtoDXF/releases)
2. Download the appropriate version for your platform:
   - Windows: `SVGtoDXF_1.0.0_x64-setup.exe`
   - macOS: `SVGtoDXF_1.0.0_x64.dmg`
   - Linux: `SVGtoDXF_1.0.0_amd64.AppImage`

### System Requirements
- **Windows**: Windows 10 or later
- **macOS**: macOS 10.15 or later
- **Linux**: Ubuntu 20.04 or equivalent

## 🎯 Usage

### Basic Conversion
1. Launch the application
2. Click "Выбрать файлы" or drag & drop SVG files
3. Click "Конвертировать" to start conversion
4. Find DXF files in the output directory

### Advanced Features
- **F12**: Activate debug mode for advanced options
- **Batch Processing**: Select multiple files for conversion
- **Custom Output**: Choose specific output directories

## 🛠️ Development

### Building from Source
```bash
# Clone repository
git clone https://github.com/TemplarD/SVGtoDXF.git
cd SVGtoDXF

# Install dependencies
cd src-tauri
cargo build

# Run development version
cargo tauri dev
```

### Architecture
- **Frontend**: HTML5, CSS3, JavaScript
- **Backend**: Rust with Tauri framework
- **Graphics**: SVG parsing and DXF generation
- **File System**: Cross-platform file management

## 📊 Features Overview

| Feature | Status | Description |
|---------|--------|-------------|
| SVG Conversion | ✅ | High-quality vector conversion |
| Batch Processing | ✅ | Multiple file conversion |
| Debug Mode | ✅ | F12 debugging tools |
| File Management | ✅ | Intelligent path handling |
| Logging System | ✅ | Daily rotation logs |
| Autotesting | ✅ | Comprehensive testing |
| UI Animations | ✅ | Modern animated interface |
| Error Recovery | ✅ | Automatic error handling |

## 🐛 Troubleshooting

### Common Issues
- **Permission Denied**: Check file system permissions
- **Conversion Failed**: Verify SVG file format
- **Missing Output**: Check destination directory

### Debug Mode
Press **F12** to access:
- System information
- File system diagnostics
- Error logs
- Performance metrics

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/TemplarD/SVGtoDXF/issues)
- **Documentation**: [Wiki](https://github.com/TemplarD/SVGtoDXF/wiki)
- **Releases**: [GitHub Releases](https://github.com/TemplarD/SVGtoDXF/releases)

## 🎉 Changelog

### v1.0.0 (2026-01-26)
- ✅ Complete SVG to DXF conversion engine
- ✅ Advanced debug mode with F12 activation
- ✅ Comprehensive file system management
- ✅ Modern animated UI with tooltips
- ✅ Batch processing capabilities
- ✅ Intelligent error handling and recovery
- ✅ Cross-platform compatibility
- ✅ Professional logging system
- ✅ Autotesting framework

---

**Made with ❤️ by TemplarD**

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
