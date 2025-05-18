use crate::audio::context::microphone_context::MicrophoneContext;
use crate::audio::context::node::Node;
use crate::device::input::microphone::Microphone;
use crate::{log_error, AppState};
use cpal::traits::{DeviceTrait, HostTrait};
use std::error::Error;
use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

pub fn list_speakers() -> Result<Vec<cpal::Device>, Box<dyn Error>> {
    let host = cpal::default_host();
    let output_devices = host.output_devices()?;
    let devices: Vec<cpal::Device> = output_devices.collect();
    Ok(devices)
}
#[allow(dead_code)]
pub fn get_speaker_by_name(device_name: &str) -> Result<cpal::Device, Box<dyn Error>> {
    match list_speakers() {
        Ok(devices) => {
            for device in devices {
                if device.name()?.eq(device_name) {
                    return Ok(device);
                }
            }
            Err(format!("Speaker '{}' not found", device_name).into())
        }
        Err(err) => Err(err),
    }
}
#[tauri::command]
pub fn list_speaker_names() -> Result<Vec<String>, String> {
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
pub fn list_microphones() -> Result<Vec<cpal::Device>, Box<dyn Error>> {
    let host = cpal::default_host();
    let input_devices = host.input_devices()?;
    let devices: Vec<cpal::Device> = input_devices.collect();
    Ok(devices)
}
pub fn get_microphone_by_name(device_name: &str) -> Result<cpal::Device, Box<dyn Error>> {
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
#[tauri::command]
pub fn list_microphone_names() -> Result<Vec<String>, String> {
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
#[tauri::command(rename_all = "snake_case")]
pub fn test_microphone(
    app_state: State<'_, AppState>,
    app_handle: AppHandle,
    device_name: String,
) -> Result<String, String> {
    match get_microphone_by_name(device_name.as_str()) {
        Ok(device) => {
            let event_name = Uuid::new_v4().to_string();
            let microphone = Microphone::new(device);
            let mut microphone_context = MicrophoneContext::new(microphone);
            match microphone_context.init() {
                Ok(receiver) => {
                    let mut stream_input_node = microphone_context.create_stream_input_node();
                    let mut gain_node = microphone_context.create_gain_control_node();
                    let mut vad_node = microphone_context.create_vad_node();
                    let receiver = stream_input_node.connect_input_source(receiver);
                    let receiver = gain_node.connect_input_source(receiver);
                    let mut receiver = vad_node.connect_input_source(receiver);
                    let microphone_gain = app_state.get_microphone_gain();
                    gain_node.set_gain(microphone_gain);
                    microphone_context.connect_stream_input_node(stream_input_node);
                    microphone_context.connect_gain_control_node(gain_node);
                    microphone_context.connect_vad_node(vad_node);
                    microphone_context.start();
                    let event_name_clone = event_name.clone();
                    tokio::spawn(async move {
                        while let Some(vad_audio_frame) = receiver.recv().await {
                            let probability = vad_audio_frame.probability();
                            
                            if let Err(err) =
                                app_handle.emit(&event_name_clone, probability)
                            {
                                log_error!("Failed to send the probability to the frontend: {}", err);
                            }
                        }
                    });
                    match app_state.set_microphone_context_test(microphone_context) {
                        Ok(_) => Ok(event_name),
                        Err(err) => Err(format!("Failed to save microphone context: {}", err)),
                    }
                }
                Err(err) => Err(format!("Failed to initialize microphone: {}", err)),
            }
        }
        Err(err) => Err(format!(
            "Microphone {} not found, error: {}",
            device_name, err
        )),
    }
}
#[tauri::command]
pub fn stop_test_microphone(app_state: State<'_, AppState>) -> Result<(), String> {
    let microphone_context_test = app_state.get_microphone_context_test();
    let mut microphone_context_test = microphone_context_test
        .lock()
        .map_err(|err| format!("Failed to lock microphone: {}", err))?;
    // Here, ownership of context is taken from app_state using take.
    // Once it's taken, app_state no longer owns context, and everything related to context will be dropped and cleaned up.
    if let Some(mut context) = microphone_context_test.take() {
        context.close();
    }
    Ok(())
}
