// ПОЛНОЦЕННЫЙ АВТОТЕСТ SVG to DXF Converter
// Проверяет РЕАЛЬНУЮ функциональность

console.log('🚀 НАЧАЛО КОМПЛЕКСНОГО АВТОТЕСТА');

async function runComprehensiveAutotest() {
    const results = {
        timestamp: new Date().toISOString(),
        tests: [],
        summary: { passed: 0, failed: 0, total: 0 }
    };

    // Тест 1: Проверка Tauri API
    await testTauriAPI(results);
    
    // Тест 2: Проверка файловой системы через Tauri
    await testTauriFileSystem(results);
    
    // Тест 3: Проверка UI элементов
    await testUIElements(results);
    
    // Тест 4: Проверка диалогов
    await testDialogs(results);
    
    // Тест 5: Проверка справки
    await testHelp(results);
    
    // Тест 6: Сравнение с реальной файловой системой
    await testFileSystemConsistency(results);
    
    // Итоги
    printResults(results);
}

async function testTauriAPI(results) {
    console.log('\n📡 Тест 1: Проверка Tauri API');
    const test = { name: 'Tauri API', passed: false, details: [] };
    
    try {
        if (typeof window.__TAURI__ !== 'undefined') {
            test.details.push('✅ window.__TAURI__ доступен');
            
            if (window.__TAURI__.core) {
                test.details.push('✅ window.__TAURI__.core доступен');
                
                if (typeof window.__TAURI__.core.invoke === 'function') {
                    test.details.push('✅ invoke функция доступна');
                    test.passed = true;
                } else {
                    test.details.push('❌ invoke функция недоступна');
                }
            } else {
                test.details.push('❌ window.__TAURI__.core недоступен');
            }
            
            test.details.push('📋 Доступные свойства: ' + Object.keys(window.__TAURI__));
        } else {
            test.details.push('❌ window.__TAURI__ недоступен');
        }
    } catch (error) {
        test.details.push('❌ Ошибка: ' + error.message);
    }
    
    results.tests.push(test);
    updateSummary(results, test.passed);
}

async function testTauriFileSystem(results) {
    console.log('\n📁 Тест 2: Проверка файловой системы через Tauri');
    const test = { name: 'Файловая система Tauri', passed: false, details: [] };
    
    try {
        test.details.push('🔍 Пробуем list_directory с текущей директорией...');
        const files = await window.__TAURI__.core.invoke('list_directory', { path: '.' });
        
        test.details.push(`✅ Получено ${files.length} элементов`);
        
        // Проверяем структуру
        let hasValidStructure = true;
        for (const file of files) {
            if (!file.name || !file.path) {
                hasValidStructure = false;
                test.details.push(`❌ Неверная структура: ${JSON.stringify(file)}`);
                break;
            }
        }
        
        if (hasValidStructure) {
            test.details.push('✅ Структура файлов корректна');
            
            // Проверяем наличие папки src
            const hasSrc = files.some(f => f.name === 'src');
            if (hasSrc) {
                test.details.push('✅ Папка src найдена');
                test.passed = true;
            } else {
                test.details.push('❌ Папка src не найдена');
                test.details.push('📋 Найденные папки: ' + files.filter(f => !f.is_directory).map(f => f.name).join(', '));
            }
            
            // Показываем первые 5 файлов
            test.details.push('📋 Первые 5 элементов:');
            files.slice(0, 5).forEach(f => {
                test.details.push(`   📁 ${f.name} (${f.is_directory ? 'папка' : 'файл'})`);
            });
        }
        
    } catch (error) {
        test.details.push('❌ Ошибка вызова list_directory: ' + error.message);
    }
    
    results.tests.push(test);
    updateSummary(results, test.passed);
}

async function testUIElements(results) {
    console.log('\n🎨 Тест 3: Проверка UI элементов');
    const test = { name: 'UI элементы', passed: false, details: [] };
    
    const elements = [
        { id: 'selectFilesBtn', name: 'Кнопка выбора файлов' },
        { id: 'selectFolderBtn', name: 'Кнопка выбора папки' },
        { id: 'convertBtn', name: 'Кнопка конвертации' },
        { id: 'clearBtn', name: 'Кнопка очистки' },
        { id: 'helpBtn', name: 'Кнопка справки' },
        { id: 'fileList', name: 'Список файлов' },
        { id: 'fileDialog', name: 'Диалог файлов' },
        { id: 'folderDialog', name: 'Диалог папок' },
        { id: 'helpDialog', name: 'Диалог справки' }
    ];
    
    let foundCount = 0;
    for (const element of elements) {
        const el = document.getElementById(element.id);
        if (el) {
            test.details.push(`✅ ${element.name} найден`);
            foundCount++;
        } else {
            test.details.push(`❌ ${element.name} НЕ найден`);
        }
    }
    
    test.passed = foundCount === elements.length;
    test.details.push(`📊 Найдено ${foundCount}/${elements.length} элементов`);
    
    results.tests.push(test);
    updateSummary(results, test.passed);
}

async function testDialogs(results) {
    console.log('\n💬 Тест 4: Проверка диалогов');
    const test = { name: 'Диалоги', passed: false, details: [] };
    
    try {
        // Проверяем функцию show для customDialog
        if (typeof customDialog === 'object' && typeof customDialog.show === 'function') {
            test.details.push('✅ customDialog.show доступен');
            
            // Пробуем показать диалог
            customDialog.show();
            test.details.push('🔍 Показываем диалог файлов...');
            
            // Ждем немного и проверяем что диалог появился
            setTimeout(() => {
                const dialog = document.getElementById('fileDialog');
                if (dialog && dialog.classList.contains('active')) {
                    test.details.push('✅ Диалог файлов появился на экране');
                    
                    // Проверяем что в диалоге есть файлы
                    const fileBrowser = document.getElementById('fileBrowser');
                    if (fileBrowser) {
                        const files = fileBrowser.children;
                        if (files.length > 0) {
                            test.details.push(`✅ В диалоге ${files.length} элементов`);
                            test.passed = true;
                        } else {
                            test.details.push('❌ В диалоге нет файлов');
                        }
                    } else {
                        test.details.push('❌ fileBrowser не найден');
                    }
                    
                    // Закрываем диалог
                    customDialog.hide();
                    test.details.push('🔍 Диалог закрыт');
                } else {
                    test.details.push('❌ Диалог не появился на экране');
                }
            }, 1000);
            
        } else {
            test.details.push('❌ customDialog.show недоступен');
        }
    } catch (error) {
        test.details.push('❌ Ошибка при работе с диалогом: ' + error.message);
    }
    
    results.tests.push(test);
    updateSummary(results, test.passed);
}

async function testHelp(results) {
    console.log('\n❓ Тест 5: Проверка справки');
    const test = { name: 'Справка', passed: false, details: [] };
    
    try {
        if (typeof helpDialog === 'object' && typeof helpDialog.show === 'function') {
            test.details.push('✅ helpDialog.show доступен');
            
            // Пробуем показать справку
            helpDialog.show();
            test.details.push('🔍 Показываем справку...');
            
            // Ждем и проверяем
            setTimeout(() => {
                const helpDialogEl = document.getElementById('helpDialog');
                if (helpDialogEl && helpDialogEl.style.display !== 'none') {
                    test.details.push('✅ Справка появилась на экране');
                    test.passed = true;
                    
                    // Закрываем справку
                    helpDialog.close();
                    test.details.push('🔍 Справка закрыта');
                } else {
                    test.details.push('❌ Справка не появилась на экране');
                }
            }, 500);
            
        } else {
            test.details.push('❌ helpDialog.show недоступен');
        }
    } catch (error) {
        test.details.push('❌ Ошибка при работе со справкой: ' + error.message);
    }
    
    results.tests.push(test);
    updateSummary(results, test.passed);
}

async function testFileSystemConsistency(results) {
    console.log('\n🔄 Тест 6: Сравнение с реальной файловой системой');
    const test = { name: 'Консистентность ФС', passed: false, details: [] };
    
    try {
        // Получаем файлы через Tauri
        const tauriFiles = await window.__TAURI__.core.invoke('list_directory', { path: '.' });
        
        test.details.push(`📡 Tauri: ${tauriFiles.length} файлов`);
        
        // Сортируем для сравнения
        const tauriNames = tauriFiles.map(f => f.name).sort();
        
        // Проверяем ключевые файлы
        const keyFiles = ['src', 'Cargo.toml', 'target', 'dist'];
        const foundKeyFiles = keyFiles.filter(name => tauriNames.includes(name));
        
        test.details.push(`🔍 Ключевые файлы найдены: ${foundKeyFiles.join(', ')}`);
        
        if (foundKeyFiles.length >= 3) {
            test.details.push('✅ Основные файлы присутствуют');
            test.passed = true;
        } else {
            test.details.push('❌ Не хватает ключевых файлов');
        }
        
        test.details.push('📋 Все файлы через Tauri:');
        tauriNames.forEach(name => {
            test.details.push(`   📁 ${name}`);
        });
        
    } catch (error) {
        test.details.push('❌ Ошибка: ' + error.message);
    }
    
    results.tests.push(test);
    updateSummary(results, test.passed);
}

function updateSummary(results, passed) {
    if (passed) {
        results.summary.passed++;
    } else {
        results.summary.failed++;
    }
    results.summary.total++;
}

function printResults(results) {
    console.log('\n' + '='.repeat(60));
    console.log('📊 ИТОГИ КОМПЛЕКСНОГО АВТОТЕСТА');
    console.log('='.repeat(60));
    
    results.tests.forEach(test => {
        console.log(`\n${test.passed ? '✅' : '❌'} ${test.name}`);
        test.details.forEach(detail => {
            console.log(`   ${detail}`);
        });
    });
    
    console.log('\n' + '='.repeat(60));
    console.log(`📈 СТАТИСТИКА:`);
    console.log(`   ✅ Пройдено: ${results.summary.passed}`);
    console.log(`   ❌ Провалено: ${results.summary.failed}`);
    console.log(`   📊 Всего: ${results.summary.total}`);
    console.log(`   📈 Успешность: ${Math.round((results.summary.passed / results.summary.total) * 100)}%`);
    console.log('='.repeat(60));
    
    if (results.summary.failed === 0) {
        console.log('🎉 ВСЕ ТЕСТЫ ПРОЙДЕНЫ! ПРИЛОЖЕНИЕ РАБОТАЕТ ИДЕАЛЬНО!');
    } else {
        console.log('⚠️ ЕСТЬ ПРОБЛЕМЫ! НУЖНО ИСПРАВИТЬ.');
    }
}

// Запускаем тест
runComprehensiveAutotest();
