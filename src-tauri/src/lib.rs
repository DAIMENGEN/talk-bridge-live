mod audio;
mod device;
mod logger;
mod silero_vad;

use crate::audio::audio_context::AudioContext;
use crate::audio::recorder::{start_recording, stop_recording};
use crate::device::device_manager::{
    human_voice_detection, list_microphone_names, list_speaker_names, set_microphone_gain,
    stop_human_voice_detection,
};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri_plugin_log::{RotationStrategy, Target, TargetKind, TimezoneStrategy};

pub struct AppState {
    microphone_gain: Arc<Mutex<f32>>,
    recording_context: Mutex<Option<AudioContext>>,
    human_voice_detection_context: Mutex<Option<AudioContext>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            microphone_gain: Arc::new(Mutex::new(1.0)),
            recording_context: None.into(),
            human_voice_detection_context: None.into(),
        })
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
                .max_file_size(524_288_000) // Limit the size of each log file to 500MB.
                .rotation_strategy(RotationStrategy::KeepAll) // Tauri can automatically rotate your log file when it reaches the size limit instead of discarding the previous file.
                .timezone_strategy(TimezoneStrategy::UseLocal) // Set the time zone strategy to use the local time zone.
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
