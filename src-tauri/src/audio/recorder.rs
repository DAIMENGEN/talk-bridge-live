use crate::device::device_manager::get_microphone_by_name;
use crate::device::input::microphone::Microphone;
use crate::{log_info, AppState};
use tauri::{AppHandle, Emitter, State};
use crate::audio::transcription::TranscriptionData;

#[tauri::command(rename_all = "snake_case")]
pub async fn start_recording(
    app: AppHandle,
    app_state: State<'_, AppState>,
    device_name: String,
) -> Result<bool, String> {
    match get_microphone_by_name(&device_name).await {
        Ok(device) => {
            let mut microphone = Microphone::new(device);
            let mut receiver = microphone.init().unwrap();
            tokio::spawn(async move {
                while let Some(samples) = receiver.recv().await {
                    log_info!("Samples length: {}", samples.len());
                    app.emit("start_recording", TranscriptionData {
                        text: "音频已经开始转录，记录音频的转录结果".to_string(),
                    }).unwrap();
                }
            });
            microphone.play();
            let mut microphone_lock = app_state.microphone.lock().unwrap();
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
pub async fn stop_recording(app_state: State<'_, AppState>) -> Result<bool, String> {
    let mut microphone_lock = app_state
        .microphone
        .lock()
        .map_err(|err| format!("Failed to lock microphone: {}", err))?;
    // Here, the take method takes ownership of the microphone, and AppState loses ownership.
    // TODO Consider whether to optimize this in the future.
    if let Some(mut microphone) = microphone_lock.take() {
        microphone.pause();
    }
    Ok(true)
}
