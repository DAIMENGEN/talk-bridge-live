pub mod hardware;

use crate::hardware::input::microphone::Microphone;
use cpal::traits::HostTrait;
use log::info;
use std::mem::forget;
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_recording() -> Result<(), ()> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    let mut microphone = Microphone::new(device);
    let mut rx = microphone.initialization().unwrap();
    microphone.start();
    tokio::spawn(async move {
        while let Some(buffer) = rx.recv().await {
            info!("length: {}", buffer.len());
        }
    });
    forget(microphone);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet, start_recording])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
