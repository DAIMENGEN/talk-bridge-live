use cpal::traits::{DeviceTrait, HostTrait};
use log::info;
use tauri_plugin_log::{Target, TargetKind};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    let device_name = device.name().unwrap();
    let sample_rate = config.sample_rate().0;
    let channels = config.channels();
    info!("channels: {}", channels);
    info!("sample_rate: {}", sample_rate);
    info!("device_name: {}", &device_name);
    format!(
        "Hello, {}! You've been greeted from Rust!, {}, {}, {}",
        name, &device_name, sample_rate, channels
    )
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
