use crate::audio::audio_context::AudioContext;
use crate::audio::audio_node::AudioNode;
use crate::device::input::microphone::Microphone;
use crate::{log_error, log_info, AppState};
use cpal::traits::{DeviceTrait, HostTrait};
use serde::Serialize;
use std::error::Error;
use tauri::{AppHandle, Emitter, State};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanVoiceProbability {
    pub probability: f32,
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
pub async fn human_voice_detection(
    app: AppHandle,
    app_state: State<'_, AppState>,
    device_name: String,
) -> Result<String, String> {
    match get_microphone_by_name(&device_name).await {
        Ok(device) => {
            const EVENT_NAME: &str = "human_voice_detection_result_event";
            let microphone = Microphone::new(device);
            let mut audio_context = AudioContext::new(microphone);
            let receiver = audio_context.init().unwrap();
            let mut source_node = audio_context.create_source_node();
            let mut gain_node = audio_context.create_gain_node();
            let mut vad_node = audio_context.create_vad_node();
            let receiver = source_node.connect_input_source(receiver);
            let receiver = gain_node.connect_input_source(receiver);
            let mut receiver = vad_node.connect_input_source(receiver);
            let microphone_gain = app_state.microphone_gain.clone();
            gain_node.set_gain(microphone_gain);
            tokio::spawn(async move {
                while let Some(vad_audio_frame) = receiver.recv().await {
                    let probability = vad_audio_frame.get_probability();
                    let samples = vad_audio_frame.get_samples();
                    log_info!("Probability: {}, Samples: {}", probability, samples.len());
                    if let Err(err) = app.emit(EVENT_NAME, HumanVoiceProbability { probability }) {
                        log_error!("Failed to send the detected human voice probability to the frontend: {}", err);
                    }
                }
            });
            audio_context.connect_source_node(source_node);
            audio_context.connect_gain_node(gain_node);
            audio_context.connect_vad_node(vad_node);
            audio_context.start();
            let mut audio_context_lock = app_state.audio_context.lock().unwrap();
            audio_context_lock.replace(audio_context);
            Ok(EVENT_NAME.parse().unwrap())
        }
        Err(err) => Err(format!(
            "Microphone {} not found, error: {}",
            device_name, err
        )),
    }
}
#[tauri::command]
pub async fn stop_human_voice_detection(app_state: State<'_, AppState>) -> Result<bool, String> {
    let mut audio_context_lock = app_state
        .audio_context
        .lock()
        .map_err(|err| format!("Failed to lock microphone: {}", err))?;
    if let Some(mut audio_context) = audio_context_lock.take() {
        audio_context.close();
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
#[tauri::command(rename_all = "snake_case")]
pub async fn set_microphone_gain(
    app_state: State<'_, AppState>,
    microphone_gain: f32,
) -> Result<bool, String> {
    let mut microphone_gain_lock = app_state
        .microphone_gain
        .lock()
        .map_err(|err| format!("Failed to lock microphone gain: {}", err))?;
    *microphone_gain_lock = microphone_gain;
    Ok(true)
}
