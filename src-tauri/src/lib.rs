mod app_state;
mod audio;
mod device;
mod logger;
mod silero_vad;
mod utils;
mod protos_gen;
mod language;
mod speech_translation;

use crate::app_state::{set_microphone_gain, set_speech_threshold, set_audio_tolerance, AppState, set_speech_merge_threshold, set_speaker, set_meeting_room, set_asr_service_url};
use crate::speech_translation::{start_recording, stop_recording};
use crate::device::device_manager::{
    human_voice_detection, list_microphone_names, list_speaker_names,
    stop_human_voice_detection,
};
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
        .invoke_handler(tauri::generate_handler![
            set_speaker,
            stop_recording,
            start_recording,
            set_meeting_room,
            list_speaker_names,
            set_asr_service_url,
            set_audio_tolerance,
            set_microphone_gain,
            set_speech_threshold,
            list_microphone_names,
            human_voice_detection,
            stop_human_voice_detection,
            set_speech_merge_threshold,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
