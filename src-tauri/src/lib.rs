use tauri::Emitter;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet_async(app: tauri::AppHandle, name: String) {
    //format!("Hello, {}! You've been greeted from Rust!", name)
    std::thread::spawn( move || {
        let greet = format!("Hello, {}! You've been greeted from Rust!", name);
        std::thread::sleep(std::time::Duration::from_secs(5));
        app.emit("greet", greet).unwrap();
    });
}

#[tauri::command]
fn greet2(name: &str) -> String{
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet_async, greet2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
