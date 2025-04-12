use crate::device::input::microphone::Microphone;
use crate::AppState;
use cpal::traits::{DeviceTrait, HostTrait};
use serde::Serialize;
use std::error::Error;
use log::info;
use tauri::{AppHandle, Emitter, State};
use crate::audio::nodes::vad_node::VadNode;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeLevel {
    pub value: f32,
}

pub fn list_speakers() -> Result<Vec<cpal::Device>, Box<dyn Error>> {
    let host = cpal::default_host();
    let output_devices = host.output_devices()?;
    let devices: Vec<cpal::Device> = output_devices.collect();
    Ok(devices)
}
pub fn list_microphones() -> Result<Vec<cpal::Device>, Box<dyn Error>> {
    let host = cpal::default_host();
    let input_devices = host.input_devices()?;
    let devices: Vec<cpal::Device> = input_devices.collect();
    Ok(devices)
}
pub async fn get_microphone_by_name(device_name: &str) -> Result<cpal::Device, Box<dyn Error>> {
    match list_microphones() {
        Ok(devices) => {
            for device in devices {
                if device.name()?.eq(device_name) {
                    return Ok(device);
                }
            }
            Err(format!("Microphone '{}' not found", device_name).into())
        }
        Err(err) => Err(err),
    }
}
#[tauri::command(rename_all = "snake_case")]
pub async fn start_microphone_test(
    app: AppHandle,
    app_state: State<'_, AppState>,
    device_name: String,
) -> Result<bool, String> {
    match get_microphone_by_name(&device_name).await {
        Ok(device) => {
            let mut microphone = Microphone::new(device);
            let mut receiver = microphone.init().unwrap();
            let speech_threshold = 0.75f32;
            let target_sample_rate = microphone.get_target_sample_rate() as u32;
            let output_frames_size = microphone.get_output_frames_size() as usize;
            tokio::spawn(async move {
                let mut vad_node = VadNode::new(target_sample_rate, output_frames_size, speech_threshold);
                while let Some(samples) = receiver.recv().await {
                    let probability = vad_node.predict(&samples);
                    if probability > 0.5 {
                        info!("probability: {}", probability);
                    }
                    app.emit("microphone_realtime_volume", VolumeLevel {
                        value: probability * 100.0,
                    }).unwrap();
                }
            });
            microphone.play();
            let mut microphone_lock = app_state.test_microphone.lock().unwrap();
            microphone_lock.replace(microphone);
            Ok(true)
        }
        Err(err) => Err(format!(
            "Microphone {} not found, error: {}",
            device_name, err
        )),
    }
}

#[tauri::command]
pub async fn stop_microphone_test(app_state: State<'_, AppState>) -> Result<bool, String> {
    let mut microphone_lock = app_state
        .test_microphone
        .lock()
        .map_err(|err| format!("Failed to lock microphone: {}", err))?;
    if let Some(mut microphone) = microphone_lock.take() {
        microphone.pause();
    }
    Ok(true)
}

#[tauri::command]
pub async fn list_microphone_names() -> Result<Vec<String>, String> {
    match list_microphones() {
        Ok(devices) => {
            let device_names = devices
                .iter()
                .filter_map(|device| device.name().ok())
                .collect::<Vec<String>>();
            Ok(device_names)
        }
        Err(err) => Err(format!("Failed to list microphone names: {}", err)),
    }
}
#[tauri::command]
pub async fn list_speaker_names() -> Result<Vec<String>, String> {
    match list_speakers() {
        Ok(devices) => {
            let device_names = devices
                .iter()
                .filter_map(|device| device.name().ok())
                .collect::<Vec<String>>();
            Ok(device_names)
        }
        Err(err) => Err(format!("Failed to list speaker names: {}", err)),
    }
}


