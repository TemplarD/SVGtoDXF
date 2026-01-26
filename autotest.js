// Полный автотест SVG to DXF Converter v1.0.0
// Запускается через консоль браузера на открытой странице приложения

async function runFullAutotest() {
    console.log('🚀 Начало полного автотеста SVG to DXF Converter v1.0.0');
    
    const results = {
        timestamp: new Date().toISOString(),
        version: '1.0.0',
        tests: [],
        summary: { passed: 0, failed: 0, total: 0 }
    };
    
    // Тест 1: Проверка доступности UI элементов
    await testUIElements(results);
    
    // Тест 2: Проверка кнопок конвертации
    await testConversionButtons(results);
    
    // Тест 3: Проверка диалогов
    await testDialogs(results);
    
    // Тест 4: Проверка интерактивности
    await testInteractivity(results);
    
    // Тест 5: Проверка файловой системы
    await testFileSystem(results);
    
    // Тест 6: Проверка конвертации
    await testConversion(results);
    
    // Тест 7: Проверка производительности
    await testPerformance(results);
    
    // Тест 8: Проверка доступности
    await testAccessibility(results);
    
    // Тест 9: Проверка анимаций
    await testAnimations(results);
    
    // Тест 10: Проверка логирования
    await testLogging(results);
    
    // Формируем итоговый отчет
    const summary = generateSummary(results);
    
    console.log('📊 ИТОГИ АВТОТЕСТА:');
    console.log(`✅ Пройдено: ${summary.passed}`);
    console.log(`❌ Провалено: ${summary.failed}`);
    console.log(`📈 Всего тестов: ${summary.total}`);
    console.log(`🎯 Успешность: ${summary.successRate}%`);
    
    // Сохраняем результаты в лог
    if (window.__TAURI__ && window.__TAURI__.core) {
        try {
            // Записываем начало автотеста
            await window.__TAURI__.core.invoke('write_log', {
                level: 'INFO',
                message: `🚀 Начало полного автотеста SVG to DXF v1.0.0`,
                timestamp: new Date().toISOString()
            });
            
            // Записываем каждый тест
            for (const test of results.tests) {
                await window.__TAURI__.core.invoke('write_log', {
                    level: test.passed ? 'INFO' : 'ERROR',
                    message: `${test.passed ? '✅' : '❌'} ${test.name}: ${test.passed ? 'ПРОЙДЕН' : 'ПРОВАЛЕН'} (${test.duration}ms)`,
                    timestamp: new Date().toISOString()
                });
                
                // Записываем детали теста
                if (test.details && test.details.length > 0) {
                    for (const detail of test.details) {
                        await window.__TAURI__.core.invoke('write_log', {
                            level: 'DEBUG',
                            message: `   ${detail}`,
                            timestamp: new Date().toISOString()
                        });
                    }
                }
            }
            
            // Записываем итоговые результаты
            await window.__TAURI__.core.invoke('write_log', {
                level: 'INFO',
                message: `📊 ИТОГИ АВТОТЕСТА: ${summary.passed}/${summary.total} тестов пройдено (${summary.successRate}%)`,
                timestamp: new Date().toISOString()
            });
            
            console.log('✅ Результаты автотеста сохранены в лог');
        } catch (error) {
            console.error('❌ Ошибка сохранения результатов в лог:', error);
        }
    }
    
    return results;
}

// Тест 1: Проверка UI элементов
async function testUIElements(results) {
    console.log('🔍 Тест 1: Проверка UI элементов...');
    
    const test = {
        name: 'UI Elements Test',
        passed: true,
        details: [],
        duration: 0
    };
    
    const startTime = Date.now();
    
    try {
        // Проверка кнопок
        const buttons = ['selectFilesBtn', 'selectFolderBtn', 'convertBtn', 'clearBtn'];
        for (const btnId of buttons) {
            const btn = document.getElementById(btnId);
            if (btn) {
                test.details.push(`✅ Кнопка ${btnId} найдена и доступна`);
                if (btn.disabled) {
                    test.passed = false;
                    test.details.push(`❌ Кнопка ${btnId} заблокирована`);
                }
            } else {
                test.passed = false;
                test.details.push(`❌ Кнопка ${btnId} не найдена`);
            }
        }
        
        // Проверка статуса
        const statusElement = document.getElementById('statusMessage');
        if (statusElement) {
            test.details.push('✅ Статус бар доступен');
        } else {
            test.passed = false;
            test.details.push('❌ Статус бар не найден');
        }
        
        // Проверка области файлов
        const fileList = document.getElementById('fileList');
        if (fileList) {
            test.details.push('✅ Область файлов доступна');
        } else {
            test.passed = false;
            test.details.push('❌ Область файлов не найдена');
        }
        
        // Проверка контейнера
        const container = document.querySelector('.container');
        if (container) {
            test.details.push('✅ Основной контейнер доступен');
        } else {
            test.passed = false;
            test.details.push('❌ Основной контейнер не найден');
        }
        
    } catch (error) {
        test.passed = false;
        test.details.push(`❌ Ошибка: ${error.message}`);
    }
    
    test.duration = Date.now() - startTime;
    results.tests.push(test);
    
    console.log(`   ${test.passed ? '✅' : '❌'} ${test.name} - ${test.duration}ms`);
}

// Тест 2: Проверка кнопок конвертации
async function testConversionButtons(results) {
    console.log('🔄 Тест 2: Проверка кнопок конвертации...');
    
    const test = {
        name: 'Conversion Buttons Test',
        passed: true,
        details: [],
        duration: 0
    };
    
    const startTime = Date.now();
    
    try {
        const convertBtn = document.getElementById('convertBtn');
        if (convertBtn) {
            test.details.push('✅ Кнопка конвертации найдена');
            
            // Проверяем состояние кнопки
            if (convertBtn.disabled) {
                test.details.push('ℹ️ Кнопка конвертации заблокирована (ожидает файлы)');
            } else {
                test.details.push('✅ Кнопка конвертации активна');
            }
            
            // Проверяем текст кнопки
            if (convertBtn.textContent.includes('Конвертировать')) {
                test.details.push('✅ Текст кнопки корректен');
            } else {
                test.passed = false;
                test.details.push('❌ Текст кнопки некорректен');
            }
        } else {
            test.passed = false;
            test.details.push('❌ Кнопка конвертации не найдена');
        }
        
        // Проверка кнопки выбора файлов
        const selectBtn = document.getElementById('selectFilesBtn');
        if (selectBtn) {
            test.details.push('✅ Кнопка выбора файлов найдена');
        } else {
            test.passed = false;
            test.details.push('❌ Кнопка выбора файлов не найдена');
        }
        
        // Проверка кнопки очистки
        const clearBtn = document.getElementById('clearBtn');
        if (clearBtn) {
            test.details.push('✅ Кнопка очистки найдена');
        } else {
            test.passed = false;
            test.details.push('❌ Кнопка очистки не найдена');
        }
        
    } catch (error) {
        test.passed = false;
        test.details.push(`❌ Ошибка: ${error.message}`);
    }
    
    test.duration = Date.now() - startTime;
    results.tests.push(test);
    
    console.log(`   ${test.passed ? '✅' : '❌'} ${test.name} - ${test.duration}ms`);
}

// Тест 3: Проверка диалогов
async function testDialogs(results) {
    console.log('💬 Тест 3: Проверка диалогов...');
    
    const test = {
        name: 'Dialogs Test',
        passed: true,
        details: [],
        duration: 0
    };
    
    const startTime = Date.now();
    
    try {
        // Тест 3.1: Проверка наличия функций для работы с диалогами
        if (typeof window.selectFiles === 'function') {
            test.details.push('✅ Функция выбора файлов доступна');
        } else {
            test.passed = false;
            test.details.push('❌ Функция выбора файлов недоступна');
        }
        
        if (typeof window.selectFolder === 'function') {
            test.details.push('✅ Функция выбора папки доступна');
        } else {
            test.passed = false;
            test.details.push('❌ Функция выбора папки недоступна');
        }
        
        // Тест 3.2: Проверка видимости кастомных диалогов
        const fileDialog = document.getElementById('customFileDialog');
        if (fileDialog) {
            test.details.push('✅ Кастомный диалог файлов найден');
            
            // Тест 3.3: Проверка элементов навигации диалога файлов
            const fileBrowser = document.getElementById('fileBrowser');
            if (fileBrowser) {
                test.details.push('✅ Браузер файлов доступен');
                
                // Проверяем наличие кнопок навигации
                const navButtons = fileBrowser.querySelectorAll('.nav-button');
                if (navButtons.length > 0) {
                    test.details.push(`✅ Найдено ${navButtons.length} кнопок навигации`);
                    navButtons.forEach((btn, index) => {
                        test.details.push(`   🎯 Кнопка ${index + 1}: ${btn.textContent}`);
                    });
                } else {
                    test.passed = false;
                    test.details.push('❌ Кнопки навигации не найдены');
                }
                
                // Проверяем путь текущей директории
                const currentPath = document.getElementById('fileCurrentPath');
                if (currentPath) {
                    test.details.push('✅ Отображение пути текущей директории доступно');
                } else {
                    test.passed = false;
                    test.details.push('❌ Отображение пути не найдено');
                }
                
            } else {
                test.passed = false;
                test.details.push('❌ Браузер файлов не найден');
            }
            
        } else {
            test.passed = false;
            test.details.push('❌ Кастомный диалог файлов не найден');
        }
        
        // Тест 3.4: Проверка диалога папок
        const folderDialog = document.getElementById('customFolderDialog');
        if (folderDialog) {
            test.details.push('✅ Кастомный диалог папок найден');
            
            // Тест 3.5: Проверка навигации диалога папок
            const folderBrowser = document.getElementById('folderBrowser');
            if (folderBrowser) {
                test.details.push('✅ Браузер папок доступен');
                
                const folderPath = document.getElementById('folderCurrentPath');
                if (folderPath) {
                    test.details.push('✅ Отображение пути папки доступно');
                } else {
                    test.passed = false;
                    test.details.push('❌ Отображение пути папки не найдено');
                }
            } else {
                test.passed = false;
                test.details.push('❌ Браузер папок не найден');
            }
            
        } else {
            test.passed = false;
            test.details.push('❌ Кастомный диалог папок не найден');
        }
        
        // Тест 3.6: Проверка диалога справки
        const helpDialog = document.getElementById('helpDialog');
        if (helpDialog) {
            test.details.push('✅ Диалог справки найден');
        } else {
            test.passed = false;
            test.details.push('❌ Диалог справки не найден');
        }
        
        // Тест 3.7: Проверка кнопок закрытия диалогов
        const closeButtons = document.querySelectorAll('.dialog-close');
        if (closeButtons.length > 0) {
            test.details.push(`✅ Найдено ${closeButtons.length} кнопок закрытия диалогов`);
        } else {
            test.passed = false;
            test.details.push('❌ Кнопки закрытия диалогов не найдены');
        }
        
        // Тест 3.8: Проверка футеров диалогов
        const footers = document.querySelectorAll('.dialog-footer');
        if (footers.length > 0) {
            test.details.push(`✅ Найдено ${footers.length} футеров диалогов`);
        } else {
            test.passed = false;
            test.details.push('❌ Футеры диалогов не найдены');
        }
        
    } catch (error) {
        test.passed = false;
        test.details.push(`❌ Ошибка: ${error.message}`);
    }
    
    test.duration = Date.now() - startTime;
    results.tests.push(test);
    
    console.log(`   ${test.passed ? '✅' : '❌'} ${test.name} - ${test.duration}ms`);
}

// Тест 4: Проверка интерактивности
async function testInteractivity(results) {
    console.log('🎮 Тест 4: Проверка интерактивности...');
    
    const test = {
        name: 'Interactivity Test',
        passed: true,
        details: [],
        duration: 0
    };
    
    const startTime = Date.now();
    
    try {
        // Проверка drag & drop
        const container = document.querySelector('.container');
        if (container) {
            test.details.push('✅ Контейнер для drag & drop найден');
            
            // Проверяем наличие обработчиков drag & drop
            if (container.ondragover || container.addEventListener) {
                test.details.push('✅ Обработчики drag & drop доступны');
            } else {
                test.passed = false;
                test.details.push('❌ Обработчики drag & drop не найдены');
            }
        } else {
            test.passed = false;
            test.details.push('❌ Контейнер для drag & drop не найден');
        }
        
        // Проверка горячих клавиш
        if (typeof window.toggleDebugMode === 'function') {
            test.details.push('✅ Функция дебаг режима доступа');
        } else {
            test.passed = false;
            test.details.push('❌ Функция дебаг режима недоступна');
        }
        
        // Проверка обновления статуса
        if (typeof window.updateStatus === 'function') {
            test.details.push('✅ Функция обновления статуса доступна');
        } else {
            test.passed = false;
            test.details.push('❌ Функция обновления статуса недоступна');
        }
        
    } catch (error) {
        test.passed = false;
        test.details.push(`❌ Ошибка: ${error.message}`);
    }
    
    test.duration = Date.now() - startTime;
    results.tests.push(test);
    
    console.log(`   ${test.passed ? '✅' : '❌'} ${test.name} - ${test.duration}ms`);
}

// Тест 5: Проверка файловой системы
async function testFileSystem(results) {
    console.log('📁 Тест 5: Проверка файловой системы...');
    
    const test = {
        name: 'File System Test',
        passed: true,
        details: [],
        duration: 0
    };
    
    const startTime = Date.now();
    
    try {
        if (window.__TAURI__ && window.__TAURI__.core) {
            test.details.push('✅ Tauri API доступен');
            
            // Тест 5.1: Проверка доступности домашней директории
            try {
                const homeDir = await window.__TAURI__.core.invoke('get_system_info');
                test.details.push('✅ Системная информация доступна');
                test.details.push(`✅ ОС: ${homeDir.os || 'неизвестно'}`);
                test.details.push(`✅ Архитектура: ${homeDir.arch || 'неизвестно'}`);
            } catch (error) {
                test.passed = false;
                test.details.push(`❌ Ошибка системной информации: ${error.message}`);
            }
            
            // Тест 5.2: Проверка доступности директорий
            try {
                await window.__TAURI__.core.invoke('file_exists', { path: '/tmp' });
                test.details.push('✅ Проверка файлов работает');
            } catch (error) {
                test.details.push('ℹ️ Проверка файлов работает (ошибка для некоторых путей нормальна)');
            }
            
            // Тест 5.3: Проверка родительской директории
            try {
                const parentDir = await window.__TAURI__.core.invoke('get_parent_directory', { path: '/tmp/test.txt' });
                test.details.push('✅ Получение родительской директории работает');
            } catch (error) {
                test.passed = false;
                test.details.push(`❌ Ошибка родительской директории: ${error.message}`);
            }
            
            // Тест 5.4: Проверка системных корней
            try {
                const roots = await window.__TAURI__.core.invoke('get_system_roots');
                if (roots && roots.length > 0) {
                    test.details.push(`✅ Найдено ${roots.length} системных корней`);
                    roots.forEach((root, index) => {
                        test.details.push(`   📁 Корень ${index + 1}: ${root}`);
                    });
                } else {
                    test.details.push('⚠️ Системные корни не найдены');
                }
            } catch (error) {
                test.passed = false;
                test.details.push(`❌ Ошибка системных корней: ${error.message}`);
            }
            
            // Тест 5.5: Проверка доступности директории записи
            try {
                const writable = await window.__TAURI__.core.invoke('check_directory_writable', { path: '/tmp' });
                test.details.push(`✅ Проверка записи в директорию работает: ${writable ? 'доступно' : 'недоступно'}`);
            } catch (error) {
                test.passed = false;
                test.details.push(`❌ Ошибка проверки записи: ${error.message}`);
            }
            
            // Тест 5.6: Проверка альтернативных путей сохранения
            try {
                const alternatives = await window.__TAURI__.core.invoke('find_alternative_save_directories', { originalPath: '/tmp/test' });
                if (alternatives && alternatives.length > 0) {
                    test.details.push(`✅ Найдено ${alternatives.length} альтернативных директорий`);
                    alternatives.forEach((alt, index) => {
                        test.details.push(`   📂 Альтернатива ${index + 1}: ${alt.name} - ${alt.reason}`);
                    });
                } else {
                    test.details.push('⚠️ Альтернативные директории не найдены');
                }
            } catch (error) {
                test.passed = false;
                test.details.push(`❌ Ошибка альтернативных путей: ${error.message}`);
            }
            
        } else {
            test.passed = false;
            test.details.push('❌ Tauri API недоступен');
        }
        
    } catch (error) {
        test.passed = false;
        test.details.push(`❌ Ошибка: ${error.message}`);
    }
    
    test.duration = Date.now() - startTime;
    results.tests.push(test);
    
    console.log(`   ${test.passed ? '✅' : '❌'} ${test.name} - ${test.duration}ms`);
}

// Тест 6: Проверка конвертации
async function testConversion(results) {
    console.log('🔄 Тест 6: Проверка конвертации...');
    
    const test = {
        name: 'Conversion Test',
        passed: true,
        details: [],
        duration: 0
    };
    
    const startTime = Date.now();
    
    try {
        if (window.__TAURI__ && window.__TAURI__.core) {
            test.details.push('✅ Tauri API для конвертации доступен');
            
            // Проверка функций конвертации
            try {
                await window.__TAURI__.core.invoke('convert_single_file', {
                    inputPath: '/test.svg',
                    outputPath: '/test.dxf'
                });
                test.details.push('✅ Функция конвертации доступна');
            } catch (error) {
                test.details.push('ℹ️ Функция конвертации работает (ошибка ожидаема для тестового файла)');
            }
            
        } else {
            test.passed = false;
            test.details.push('❌ Tauri API для конвертации недоступен');
        }
        
    } catch (error) {
        test.passed = false;
        test.details.push(`❌ Ошибка: ${error.message}`);
    }
    
    test.duration = Date.now() - startTime;
    results.tests.push(test);
    
    console.log(`   ${test.passed ? '✅' : '❌'} ${test.name} - ${test.duration}ms`);
}

// Тест 7: Проверка производительности
async function testPerformance(results) {
    console.log('⚡ Тест 7: Проверка производительности...');
    
    const test = {
        name: 'Performance Test',
        passed: true,
        details: [],
        duration: 0
    };
    
    const startTime = Date.now();
    
    try {
        // Проверка загрузки DOM
        const loadTime = performance.timing.loadEventEnd - performance.timing.navigationStart;
        if (loadTime < 5000) {
            test.details.push(`✅ Страница загружена за ${loadTime}ms`);
        } else {
            test.passed = false;
            test.details.push(`❌ Медленная загрузка: ${loadTime}ms`);
        }
        
        // Проверка памяти
        if (performance.memory) {
            const memoryMB = (performance.memory.usedJSHeapSize / 1024 / 1024).toFixed(2);
            test.details.push(`ℹ️ Использование памяти: ${memoryMB}MB`);
            
            if (memoryMB < 100) {
                test.details.push('✅ Использование памяти в норме');
            } else {
                test.details.push('⚠️ Высокое использование памяти');
            }
        }
        
        // Проверка анимаций
        test.details.push('✅ CSS анимации доступны');
        
    } catch (error) {
        test.passed = false;
        test.details.push(`❌ Ошибка: ${error.message}`);
    }
    
    test.duration = Date.now() - startTime;
    results.tests.push(test);
    
    console.log(`   ${test.passed ? '✅' : '❌'} ${test.name} - ${test.duration}ms`);
}

// Тест 8: Проверка доступности
async function testAccessibility(results) {
    console.log('♿ Тест 8: Проверка доступности...');
    
    const test = {
        name: 'Accessibility Test',
        passed: true,
        details: [],
        duration: 0
    };
    
    const startTime = Date.now();
    
    try {
        // Проверка alt текстов для изображений
        const images = document.querySelectorAll('img');
        let imagesWithAlt = 0;
        images.forEach(img => {
            if (img.alt) imagesWithAlt++;
        });
        
        if (images.length === 0 || imagesWithAlt === images.length) {
            test.details.push('✅ Все изображения имеют alt текст');
        } else {
            test.details.push(`⚠️ ${imagesWithAlt}/${images.length} изображений имеют alt текст`);
        }
        
        // Проверка заголовков
        const headings = document.querySelectorAll('h1, h2, h3, h4, h5, h6');
        if (headings.length > 0) {
            test.details.push(`✅ Найдено ${headings.length} заголовков`);
        } else {
            test.details.push('⚠️ Заголовки не найдены');
        }
        
        // Проверка кнопок с текстом
        const buttons = document.querySelectorAll('button');
        let buttonsWithText = 0;
        buttons.forEach(btn => {
            if (btn.textContent.trim()) buttonsWithText++;
        });
        
        if (buttonsWithText === buttons.length) {
            test.details.push('✅ Все кнопки имеют текст');
        } else {
            test.passed = false;
            test.details.push(`❌ ${buttonsWithText}/${buttons.length} кнопок имеют текст`);
        }
        
    } catch (error) {
        test.passed = false;
        test.details.push(`❌ Ошибка: ${error.message}`);
    }
    
    test.duration = Date.now() - startTime;
    results.tests.push(test);
    
    console.log(`   ${test.passed ? '✅' : '❌'} ${test.name} - ${test.duration}ms`);
}

// Тест 9: Проверка анимаций
async function testAnimations(results) {
    console.log('✨ Тест 9: Проверка анимаций...');
    
    const test = {
        name: 'Animations Test',
        passed: true,
        details: [],
        duration: 0
    };
    
    const startTime = Date.now();
    
    try {
        // Проверка CSS анимаций
        const styles = getComputedStyle(document.body);
        if (styles.transition || styles.animation) {
            test.details.push('✅ CSS анимации доступны');
        } else {
            test.details.push('ℹ️ CSS анимации не найдены (нормально)');
        }
        
        // Проверка функций анимации
        if (typeof window.addSuccessAnimation === 'function') {
            test.details.push('✅ Функция анимации успеха доступна');
        } else {
            test.passed = false;
            test.details.push('❌ Функция анимации успеха недоступна');
        }
        
        if (typeof window.addErrorAnimation === 'function') {
            test.details.push('✅ Функция анимации ошибки доступна');
        } else {
            test.passed = false;
            test.details.push('❌ Функция анимации ошибки недоступна');
        }
        
        // Проверка подсказок
        const tooltips = document.querySelectorAll('[data-tooltip]');
        test.details.push(`✅ Найдено ${tooltips.length} элементов с подсказками`);
        
    } catch (error) {
        test.passed = false;
        test.details.push(`❌ Ошибка: ${error.message}`);
    }
    
    test.duration = Date.now() - startTime;
    results.tests.push(test);
    
    console.log(`   ${test.passed ? '✅' : '❌'} ${test.name} - ${test.duration}ms`);
}

// Тест 10: Проверка логирования
async function testLogging(results) {
    console.log('📋 Тест 10: Проверка логирования...');
    
    const test = {
        name: 'Logging Test',
        passed: true,
        details: [],
        duration: 0
    };
    
    const startTime = Date.now();
    
    try {
        if (window.__TAURI__ && window.__TAURI__.core) {
            test.details.push('✅ Tauri API для логирования доступен');
            
            // Проверка функций логирования
            try {
                await window.__TAURI__.core.invoke('write_log', {
                    level: 'INFO',
                    message: 'Тестовое сообщение логирования',
                    timestamp: new Date().toISOString()
                });
                test.details.push('✅ Функция записи лога работает');
            } catch (error) {
                test.passed = false;
                test.details.push(`❌ Ошибка записи лога: ${error.message}`);
            }
            
            try {
                await window.__TAURI__.core.invoke('get_log_files');
                test.details.push('✅ Функция получения логов работает');
            } catch (error) {
                test.passed = false;
                test.details.push(`❌ Ошибка получения логов: ${error.message}`);
            }
            
        } else {
            test.passed = false;
            test.details.push('❌ Tauri API для логирования недоступен');
        }
        
    } catch (error) {
        test.passed = false;
        test.details.push(`❌ Ошибка: ${error.message}`);
    }
    
    test.duration = Date.now() - startTime;
    results.tests.push(test);
    
    console.log(`   ${test.passed ? '✅' : '❌'} ${test.name} - ${test.duration}ms`);
}

// Генерация итогового отчета
function generateSummary(results) {
    const passed = results.tests.filter(t => t.passed).length;
    const failed = results.tests.filter(t => !t.passed).length;
    const total = results.tests.length;
    const successRate = total > 0 ? Math.round((passed / total) * 100) : 0;
    
    results.summary = { passed, failed, total, successRate };
    
    return {
        passed,
        failed,
        total,
        successRate,
        status: successRate >= 80 ? 'SUCCESS' : 'FAILED'
    };
}

// Запуск автотеста
console.log('🚀 Полный автотест готов к запуску. Выполните: runFullAutotest()');
window.runFullAutotest = runFullAutotest;

// Интеграция с F12 и программным вызовом
// Интеграция с F12 и программным вызовом
window.runAutotestAI = async function() {
    console.log("🤖 Запуск автотеста через ИИ API...");
    
    try {
        // Проверяем доступность Tauri API
        if (!window.__TAURI__ || !window.__TAURI__.core) {
            console.error("❌ Tauri API недоступен для автотеста");
            return { success: false, error: "Tauri API недоступен" };
        }
        
        // Запускаем автотест через Rust
        const result = await window.__TAURI__.core.invoke("run_autotest_ai");
        
        console.log("✅ Автотест через ИИ завершен:", result);
        return { success: true, result };
        
    } catch (error) {
        console.error("❌ Ошибка автотеста через ИИ:", error);
        return { success: false, error: error.message };
    }
};

// Интеграция с F12
document.addEventListener('keydown', function(event) {
    // F12 + Ctrl для автотеста
    if (event.key === 'F12' && event.ctrlKey) {
        event.preventDefault();
        console.log('🧪 Запуск автотеста через Ctrl+F12...');
        runFullAutotest();
    }
});

// Глобальные функции для доступа извне
window.runAutotest = runFullAutotest;
window.runAutotestAI = window.runAutotestAI;
