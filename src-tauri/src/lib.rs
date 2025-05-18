mod app_state;
mod audio;
mod device;
mod language;
mod logger;
mod protos_gen;
mod silero_vad;
mod speech_translation;
mod utils;

use crate::app_state::{
    set_meeting_room, set_microphone_gain, set_silence_streak_threshold, set_speaker,
    set_speech_threshold, AppState,
};
use crate::device::device_manager::{
    list_microphone_names, list_speaker_names, test_microphone, stop_test_microphone,
};
use crate::speech_translation::{start_recording, stop_recording};
use std::panic;
use std::path::PathBuf;
use tauri_plugin_log::{RotationStrategy, Target, TargetKind, TimezoneStrategy};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
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
        .setup(|_app| {
            panic::set_hook(Box::new(|panic_info| {
                log_error!("{:#?}", panic_info);
            }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_speaker,
            stop_recording,
            start_recording,
            set_meeting_room,
            set_microphone_gain,
            set_speech_threshold,
            set_silence_streak_threshold,
            list_speaker_names,
            test_microphone,
            stop_test_microphone,
            list_microphone_names,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
