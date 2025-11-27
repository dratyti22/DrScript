pub fn format_runtime_error(error: &str) -> String {
    format!("🚨 Ошибка выполнения: {}", error)
}

pub fn handle_panic() -> String {
    "🚨 Произошла критическая ошибка при выполнении программы".to_string()
}