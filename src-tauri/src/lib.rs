pub mod device;
mod audio;
mod silero_vad;

use crate::audio::recorder::{start_recording, stop_recording};
use crate::device::input::microphone::Microphone;
use std::sync::Mutex;
use tauri_plugin_log::{Target, TargetKind};
use crate::device::device_manager::{list_speaker_names, list_microphone_names, start_microphone_test, stop_microphone_test};

pub struct AppState {
    microphone: Mutex<Option<Microphone>>,
    test_microphone: Mutex<Option<Microphone>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState { microphone: None.into(), test_microphone: None.into() })
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
        .invoke_handler(tauri::generate_handler![
            stop_recording,
            start_recording,
            list_speaker_names,
            list_microphone_names,
            start_microphone_test,
            stop_microphone_test,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
