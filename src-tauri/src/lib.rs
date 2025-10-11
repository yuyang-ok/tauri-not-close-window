// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use std::{backtrace::Backtrace, ops::DerefMut, panic};

use tauri::{ipc::Channel, WebviewUrl, WebviewWindowBuilder};
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
                if let Err(err) = x.send(s) {
                    println!("send error:{}", err);

                    *sender = None;
                }
            }
            None => {
                // println!("no log channel found");
            }
        }
    }

    fn flush(&self) {}
}

fn is_send<T: Send>(_x: T) {}

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
    *log_channel_store().lock().unwrap() = Some(on_event.clone());
    is_send(on_event.clone());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logger_test();

    let setup = move |app: &mut tauri::App| {
        {
            let old = panic::take_hook();
            let handler = app.handle().clone();
            panic::set_hook(Box::new(move |info| {
                let backtrace = Backtrace::capture();
                println!("panic here:{}", backtrace);
                if let Err(err) = WebviewWindowBuilder::new(
                    &handler,
                    "backend_crash",
                    WebviewUrl::App(format!("backend_crash.html").into()),
                )
                .title("Backend Crash")
                .inner_size(800f64, 600f64)
                .maximized(true)
                .build()
                {
                    //  can't use log here
                    println!("create backend crash window failed:{}", err);
                }
                old(info);
            }));
        }
        Ok(())
    };
    log::set_boxed_logger(Box::new(AppLogger {})).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, log_channel])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
