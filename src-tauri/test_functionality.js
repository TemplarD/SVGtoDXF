// Тестовый скрипт для проверки функциональности
console.log('�� Запуск тестирования функциональности...');

// Проверяем доступность Tauri API
if (window.__TAURI__ && window.__TAURI__.core) {
    console.log('✅ Tauri API доступен');
    
    // Тестируем run_autotest_ai
    window.__TAURI__.core.invoke('run_autotest_ai')
        .then(result => {
            console.log('✅ Автотест через Rust успешно выполнен:', result);
            
            // Проверяем результаты
            if (result.passed) {
                console.log('✅ Все тесты пройдены!');
            } else {
                console.log('⚠️ Некоторые тесты не пройдены:', result.message);
            }
        })
        .catch(error => {
            console.error('❌ Ошибка выполнения автотеста:', error);
        });
        
    // Тестируем файловую систему
    window.__TAURI__.core.invoke('get_system_info')
        .then(info => {
            console.log('✅ Системная информация получена:', info);
        })
        .catch(error => {
            console.error('❌ Ошибка получения системной информации:', error);
        });
        
} else {
    console.error('❌ Tauri API недоступен');
}

// Проверяем UI элементы
setTimeout(() => {
    const buttons = document.querySelectorAll('button');
    console.log(`🔍 Найдено кнопок: ${buttons.length}`);
    
    buttons.forEach((button, index) => {
        console.log(`Кнопка ${index}: ${button.textContent}`);
    });
    
    // Проверяем список файлов
    const fileList = document.getElementById('fileList');
    if (fileList) {
        console.log('✅ Список файлов найден');
    } else {
        console.log('❌ Список файлов не найден');
    }
    
}, 1000);
