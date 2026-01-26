# SVG to DXF Converter - macOS версия

## 🍎 Сборка для macOS

Для сборки macOS версии требуется macOS устройство с Xcode.

### Команды для сборки на macOS:

```bash
# Установка таргетов
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Компиляция
rustc cross_release.rs --target x86_64-apple-darwin -O -o svg-to-dxf-converter-macos-x64
rustc cross_release.rs --target aarch64-apple-darwin -O -o svg-to-dxf-converter-macos-arm64

# Создание универсального бинарника
lipo -create svg-to-dxf-converter-macos-x64 svg-to-dxf-converter-macos-arm64 -output svg-to-dxf-converter-macos-universal
```

### Требования:
- macOS 10.12 или выше
- Xcode с Command Line Tools
- Rust 1.70+

### Результат:
- Работающая файловая система
- Кросс-платформенность
- Нативная производительность

**Сборка возможна только на macOS устройстве из-за требований Apple SDK.**
