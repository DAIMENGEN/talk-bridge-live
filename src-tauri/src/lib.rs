pub mod hardware;

use std::sync::Mutex;
use crate::hardware::input::microphone::Microphone;
use cpal::traits::HostTrait;
use log::info;
use tauri::State;
use tauri_plugin_log::{Target, TargetKind};

struct AppState {
    microphone: Mutex<Option<Microphone>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_recording(app_state: State<'_, AppState>) -> Result<(), ()> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    let mut microphone = Microphone::new(device);
    let mut receiver = microphone.initialization().unwrap();
    microphone.play();
    tokio::spawn(async move {
        while let Some(buffer) = receiver.recv().await {
            info!("length: {}", buffer.len());
        }
    });
    let mut mic_lock = app_state.microphone.lock().unwrap();
    *mic_lock = Some(microphone);
    Ok(())
}

#[tauri::command]
async fn stop_recording(app_state: State<'_, AppState>) -> Result<(), ()> {
    let mut mic_lock = app_state.microphone.lock().unwrap();
    if let Some(mut mic) = mic_lock.take() {
        mic.pause();
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState { microphone: None.into() })
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
        .invoke_handler(tauri::generate_handler![greet, start_recording, stop_recording])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
