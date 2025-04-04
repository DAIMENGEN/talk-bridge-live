use crate::hardware::input::microphone::Microphone;
use crate::AppState;
use cpal::traits::HostTrait;
use log::info;
use tauri::State;

#[tauri::command]
pub async fn start_recording(app_state: State<'_, AppState>) -> Result<(), ()> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    let mut microphone = Microphone::new(device);
    let mut receiver = microphone.init().unwrap();
    tokio::spawn(async move {
        while let Some(buffer) = receiver.recv().await {
            info!("length: {}", buffer.len());
        }
    });
    microphone.play();
    let mut microphone_lock = app_state.microphone.lock().unwrap();
    microphone_lock.replace(microphone);
    Ok(())
}
#[tauri::command]
pub async fn stop_recording(app_state: State<'_, AppState>) -> Result<(), ()> {
    let mut microphone_lock = app_state.microphone.lock().unwrap();
    if let Some(mut microphone) = microphone_lock.take() {
        microphone.pause();
    }
    Ok(())
}

