// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tauri::command]
fn run_code(code: String) -> String {
    // Пока что простая заглушка - нужно будет подключить dr_script как зависимость
    // format!("Executed: {}", code)
    dr_script::run_source(code.as_str())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_code])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
