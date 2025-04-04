use crate::device::device_manager::get_microphone_by_name;
use crate::device::input::microphone::Microphone;
use crate::AppState;
use log::info;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub async fn start_recording(
    app_state: State<'_, AppState>,
    device_name: String,
) -> Result<bool, String> {
    match get_microphone_by_name(&device_name).await {
        Ok(device) => {
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
            Ok(true)
        }
        Err(err) => Err(format!(
            "Microphone {} not found, error: {}",
            device_name, err
        )),
    }
}
#[tauri::command]
pub async fn stop_recording(app_state: State<'_, AppState>) -> Result<(), ()> {
    let mut microphone_lock = app_state.microphone.lock().unwrap();
    if let Some(mut microphone) = microphone_lock.take() {
        microphone.pause();
    }
    Ok(())
}
