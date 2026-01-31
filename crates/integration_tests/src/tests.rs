//! Unit тесты для integration_tests модуля

#[cfg(test)]
mod tests {
    use crate::TestResult;

    #[test]
    fn test_test_result_creation() {
        let result = TestResult {
            name: "Test".to_string(),
            passed: true,
            message: "Success".to_string(),
        };
        
        assert_eq!(result.name, "Test");
        assert!(result.passed);
        assert_eq!(result.message, "Success");
    }

    #[test]
    fn test_test_result_serialization() {
        let result = TestResult {
            name: "Test".to_string(),
            passed: true,
            message: "Success".to_string(),
        };
        
        // Проверяем что структура может быть сериализована
        let json = serde_json::to_string(&result).unwrap();
        let _deserialized: TestResult = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_multiple_test_results() {
        let results = vec![
            TestResult {
                name: "Test 1".to_string(),
                passed: true,
                message: "Success 1".to_string(),
            },
            TestResult {
                name: "Test 2".to_string(),
                passed: false,
                message: "Error 2".to_string(),
            },
        ];
        
        assert_eq!(results.len(), 2);
        assert!(results[0].passed);
        assert!(!results[1].passed);
    }
}
