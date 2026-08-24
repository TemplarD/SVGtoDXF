# Сборка SVGtoDXF для Windows (x86_64)
# Запускать в PowerShell от имени обычного пользователя (НЕ админа обязательно,
# иначе NSIS currentUser-установка может вести себя некорректно).
#
# Требования (ставятся одинаково для сборки и подписи):
#   - Rust через rustup:        rustup target add x86_64-pc-windows-msvc
#   - Visual Studio 2022 Build Tools (нагрузка "Разработка классических
#     приложений на C++") либо отдельно: MSVC + Windows 11 SDK
#   - WebView2 (в Windows 11 уже есть; для Win10 — поставить runtime)
#   - Node.js 18+ (для trunk):  npm install -g trunk
#   - NSIS (для цели nsis; tauri-cli подтянет, но лучше поставить NSIS 3.x)
#
# ── Как убрать ругань SmartScreen на установщике ──────────────────────────────
# SmartScreen ругается на неподписанные/новые сертификаты EXE. Что делает проект:
#   1) В tauri.conf.json -> bundle.windows задан timestampUrl (http://timestamp.digicert.com)
#      и digestAlgorithm=sha256 — это встраивает в подпись метку времени, поэтому
#      подпись остаётся валидной после истечения сертификата, и SmartScreen
#      «накапливает репутацию» корректно.
#   2) installMode=currentUser — ставит без прав админа (меньше триггеров UAC/SmartScreen).
#
# Для ПОЛНОЙ тишины SmartScreen нужна подпись Authenticode (OV или EV-сертификат):
#   - OV-сертификат (файл .pfx):
#       $env:TAURI_SIGNING_PRIVATE_KEY = "путь/к/certificate.pfx"
#       $env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = "пароль"
#   - либо сертификат в хранилище Windows: прописать bundle.windows.certificateThumbprint
#     (отпечаток в tauri.conf.json, значение null сейчас = подпись не применяется).
#   При наличии валидной подписи SmartScreen не показывает предупреждение.
#
# ── Как убрать консольное окно при запуске ───────────────────────────────────
# В .cargo/config.toml прописан [target.x86_64-pc-windows-msvc] windows_subsystem="windows",
# поэтому бинарь собирается как GUI-приложение — чёрное окно консоли не появляется,
# а закрытие приложения не привязано к консоли.

cargo tauri build --target x86_64-pc-windows-msvc
Write-Host "==> Готово. Бандлы в: target/x86_64-pc-windows-msvc/release/bundle/"
