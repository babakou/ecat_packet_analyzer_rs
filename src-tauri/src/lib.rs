mod ecat_datagram;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn greet2(name: &str) -> Vec<ecat_datagram::DLInfo> {
    let dlinfos = ecat_datagram::parse_byte_array(name);

    dlinfos
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
