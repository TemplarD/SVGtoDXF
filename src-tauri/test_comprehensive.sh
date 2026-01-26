#!/bin/bash

# echo "Пропуск проверки процесса"
# echo "Пропуск проверки процесса"
# echo "Пропуск проверки процесса"
# echo "Пропуск проверки процесса"
# echo "Пропуск проверки процесса"
# echo "Пропуск проверки процесса"
# echo "Пропуск проверки процесса"
else
    echo "❌ Приложение не запущено"
    exit 1
fi

# Проверяем логи
echo "2. 📋 Проверка системы логирования..."
if [ -f "$HOME/svg-to-dxf-converter/logs/svg-to-dxf-$(date +%Y-%m-%d).log" ]; then
    echo "✅ Система логирования работает"
    echo "Последние записи в логе:"
    tail -3 "$HOME/svg-to-dxf-converter/logs/svg-to-dxf-$(date +%Y-%m-%d).log"
else
    echo "❌ Лог файл не найден"
fi

# Проверяем тестовый файл
echo "3. 📋 Проверка тестового SVG файла..."
if [ -f "test.svg" ]; then
    echo "✅ Тестовый SVG файл создан"
    echo "Содержимое файла:"
    cat test.svg
else
    echo "❌ Тестовый SVG файл не найден"
fi

# Проверяем структуру проекта
echo "4. 📋 Проверка структуры проекта..."
echo "Файлы в dist/:"
ls -la dist/ | head -5
echo "Файлы в src-tauri/src/:"
ls -la src-tauri/src/ | head -5

# Проверяем автотест
echo "5. 📋 Проверка автотеста..."
if [ -f "autotest.js" ]; then
    echo "✅ Файл автотеста найден"
    echo "Размер файла: $(wc -l < autotest.js) строк"
else
    echo "❌ Файл автотеста не найден"
fi

# Проверяем HTML структуру
echo "6. 📋 Проверка HTML структуры..."
if grep -q "fileList" dist/index.html; then
    echo "✅ Элемент fileList найден в HTML"
else
    echo "❌ Элемент fileList не найден в HTML"
fi

if grep -q "selectFilesBtn" dist/index.html; then
    echo "✅ Кнопка выбора файлов найдена в HTML"
else
    echo "❌ Кнопка выбора файлов не найдена в HTML"
fi

if grep -q "run_autotest_ai" dist/index.html; then
    echo "✅ Функция run_autotest_ai найдена в HTML"
else
    echo "❌ Функция run_autotest_ai не найдена в HTML"
fi

echo "=================================================="
echo "🎯 ТЕСТИРОВАНИЕ ЗАВЕРШЕНО"
echo "=================================================="
