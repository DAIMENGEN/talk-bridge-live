use crate::audio::audio_context::AudioContext;
use crate::audio::audio_node::AudioNode;
use crate::audio::transcription::TranscriptData;
use crate::device::device_manager::get_microphone_by_name;
use crate::device::input::microphone::Microphone;
use crate::{log_error, AppState};
use chrono::Local;
use tauri::{AppHandle, Emitter, State};

#[tauri::command(rename_all = "snake_case")]
pub async fn start_recording(
    app: AppHandle,
    app_state: State<'_, AppState>,
    device_name: String,
) -> Result<String, String> {
    match get_microphone_by_name(&device_name).await {
        Ok(device) => {
            const EVENT_NAME: &str = "transcript_result_event";
            let microphone = Microphone::new(device);
            let mut audio_context = AudioContext::new(microphone);
            let receiver = audio_context.init().unwrap();
            let mut source_node = audio_context.create_source_node();
            let mut gain_node = audio_context.create_gain_node();
            let mut vad_node = audio_context.create_vad_node();
            let mut assembler_node = audio_context.create_assembler_node();
            let receiver = source_node.connect_input_source(receiver);
            let receiver = gain_node.connect_input_source(receiver);
            let receiver = vad_node.connect_input_source(receiver);
            let mut receiver = assembler_node.connect_input_source(receiver);
            let microphone_gain = app_state.get_microphone_gain().clone();
            gain_node.set_gain(microphone_gain);
            tokio::spawn(async move {
                while let Some(samples) = receiver.recv().await {
                    let datetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    let transcript = format!("音频已经开始转录，记录音频的样本长度: {}", samples.len());
                    if let Err(err) =
                        app.emit(EVENT_NAME, TranscriptData::new(datetime, transcript))
                    {
                        log_error!(
                                "Failed to send the transcript result to the frontend: {}",
                                err
                            );
                    }
                }
            });
            audio_context.connect_source_node(source_node);
            audio_context.connect_gain_node(gain_node);
            audio_context.connect_vad_node(vad_node);
            audio_context.connect_assembler_node(assembler_node);
            audio_context.start();
            match app_state.set_recording_context(audio_context) {
                Ok(_) => Ok(EVENT_NAME.parse().unwrap()),
                Err(err) => Err(format!("Failed to save recording context: {}", err)),
            }
        }
        Err(err) => Err(format!(
            "Microphone {} not found, error: {}",
            device_name, err
        )),
    }
}

#[tauri::command]
pub async fn stop_recording(app_state: State<'_, AppState>) -> Result<bool, String> {
    let recording_context = app_state.get_recording_context();
    let mut recording_context_lock = recording_context
        .lock()
        .map_err(|err| format!("Failed to lock microphone: {}", err))?;
    // Here, ownership of context is taken from app_state using take.
    // Once it's taken, app_state no longer owns context, and everything related to context will be dropped and cleaned up.
    if let Some(context) = recording_context_lock.take() {
        context.close();
    }
    Ok(true)
}
