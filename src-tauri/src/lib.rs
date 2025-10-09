// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::time::Duration;

use tauri::ipc::Channel;

#[tauri::command]
fn download(on_event: Channel<String>) {
    let mut i = 0;
    loop {
        on_event.send(format!("downloading {}", i)).unwrap();
        i += 1;
        println!("{}", i);
        std::thread::sleep(Duration::from_secs(3));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
