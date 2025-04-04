use cpal::traits::{DeviceTrait, HostTrait};
use std::error::Error;

pub fn list_microphones() -> Result<Vec<cpal::Device>, Box<dyn Error>> {
    let host = cpal::default_host();
    let input_devices = host.input_devices()?;
    let devices: Vec<cpal::Device> = input_devices.collect();
    Ok(devices)
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
