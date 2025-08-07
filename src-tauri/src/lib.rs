use tauri::{Emitter, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet_async(app: tauri::AppHandle, name: String) {
    //format!("Hello, {}! You've been greeted from Rust!", name)
    // std::thread::spawn( move || {
    //     greet_async_sub(app, name);
    // });
    let tx : tauri::State<std::sync::mpsc::Sender<String>> = app.state();
    tx.send(name).unwrap();
}

fn greet_async_sub(app: &tauri::AppHandle, name: String) {
    let greet = format!("Hello, {}! You've been greeted from Rust!", name);
    std::thread::sleep(std::time::Duration::from_secs(3));
    app.emit("greet", greet).unwrap();
}

#[tauri::command]
fn greet2(name: &str) -> String{
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            let (tx, rx) = std::sync::mpsc::channel();
            let _join_handle = std::thread::spawn( move || {
                loop {
                    let received = rx.recv().unwrap();
                    greet_async_sub(&app_handle, received);
                }
            });
            app.manage(tx);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet_async, greet2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
