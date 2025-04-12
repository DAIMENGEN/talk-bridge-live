mod device;
mod audio;
mod silero_vad;
mod logger;

use std::path::PathBuf;
use crate::audio::recorder::{start_recording, stop_recording};
use crate::device::input::microphone::Microphone;
use std::sync::{Arc, Mutex};
use tauri_plugin_log::{RotationStrategy, Target, TargetKind, TimezoneStrategy};
use crate::device::device_manager::{list_speaker_names, list_microphone_names, human_voice_detection, stop_human_voice_detection, set_microphone_gain};

pub struct AppState {
    microphone: Mutex<Option<Microphone>>,
    microphone_gain: Arc<Mutex<f32>>,
    test_microphone: Mutex<Option<Microphone>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState { microphone: None.into(), microphone_gain: Arc::new(Mutex::new(1.0)), test_microphone: None.into() })
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Folder {
                        path: PathBuf::from("logs"),
                        file_name: None,

                    }),
                    Target::new(TargetKind::Webview),
                ])
                .max_file_size(524_288_000)// Limit the size of each log file to 500MB.
                .rotation_strategy(RotationStrategy::KeepAll)// Tauri can automatically rotate your log file when it reaches the size limit instead of discarding the previous file.
                .timezone_strategy(TimezoneStrategy::UseLocal)// Set the time zone strategy to use the local time zone.
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            stop_recording,
            start_recording,
            list_speaker_names,
            set_microphone_gain,
            list_microphone_names,
            human_voice_detection,
            stop_human_voice_detection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
