// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use std::ops::DerefMut;

use tauri::ipc::Channel;
pub fn log_channel_store() -> &'static std::sync::Mutex<Option<Channel<String>>> {
    static X: std::sync::Mutex<Option<Channel<String>>> = std::sync::Mutex::new(None);
    &X
}

pub struct AppLogger {}

impl AppLogger {
    fn get_level(&self) -> log::Level {
        log::Level::Trace
    }
}

impl log::Log for AppLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.get_level()
    }
    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let s = format!(
            "{} {} {}-{}\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            if cfg!(debug_assertions) {
                format!(
                    "{}:{}",
                    record.file().unwrap_or_default(),
                    record.line().unwrap_or_default(),
                )
            } else {
                "".to_string()
            },
            record.level(),
            record.args()
        );
        let mut sender = log_channel_store().lock().unwrap();
        match sender.deref_mut() {
            Some(x) => {
                x.send(s).unwrap();
            }
            None => {
                // println!("no log channel found");
            }
        }
    }

    fn flush(&self) {}
}

#[allow(dead_code)]
fn logger_test() {
    std::thread::spawn(|| loop {
        log::trace!("this is trace log");
        log::debug!("this is debug log");
        log::info!("this is info log");
        log::warn!("this is warn log");
        log::error!("this is error log");
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
}

#[tauri::command]
fn log_channel(on_event: Channel<String>) {
    *log_channel_store().lock().unwrap() = Some(on_event);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logger_test();
    log::set_boxed_logger(Box::new(AppLogger {})).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, log_channel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
