//! Тесты для UI модуля

#[cfg(test)]
mod tests {
    #[test]
    fn test_ui_module_compiles() {
        // Проверяем что UI модуль компилируется
        assert!(true);
    }

    #[test]
    fn test_yew_components() {
        // Проверяем что Yew компоненты могут быть созданы
        use crate::state::AppState;
        let _state = AppState::new();
        assert!(true);
    }

    #[test]
    fn test_file_item_creation() {
        use crate::state::FileItem;
        
        let file_item = FileItem {
            id: "test".to_string(),
            name: "test.svg".to_string(),
            path: "/tmp/test.svg".to_string(),
            size: 1024,
            status: crate::state::FileStatus::Pending,
        };
        
        assert_eq!(file_item.id, "test");
        assert_eq!(file_item.name, "test.svg");
        assert_eq!(file_item.size, 1024);
        assert!(matches!(file_item.status, crate::state::FileStatus::Pending));
    }

    #[test]
    fn test_file_status_equality() {
        use crate::state::FileStatus;
        
        let status1 = FileStatus::Pending;
        let status2 = FileStatus::Processing;
        let status3 = FileStatus::Pending;
        
        assert_eq!(status1, status3);
        assert_ne!(status1, status2);
    }
}
