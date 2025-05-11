use crate::audio::context::AudioContext;
use crate::audio::node::AudioNode;
use crate::audio::transcription::TranscriptData;
use crate::device::device_manager::get_microphone_by_name;
use crate::device::input::microphone::Microphone;
use crate::{log_error, AppState};
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
            let mut stream_input_node = audio_context.create_stream_input_node();
            let mut gain_control_node = audio_context.create_gain_control_node();
            let mut vad_node = audio_context.create_vad_node();
            let mut vocal_isolation_node = audio_context.create_vocal_isolation_node();
            let mut concatenation_node = audio_context.create_concatenation_node();
            let mut persistence_node = audio_context.create_persistence_node();
            let mut speech_recognition_node = audio_context.create_speech_recognition_node();
            let mut text_translation_node = audio_context.create_text_translation_node();
            let receiver = stream_input_node.connect_input_source(receiver);
            let receiver = gain_control_node.connect_input_source(receiver);
            let receiver = vad_node.connect_input_source(receiver);
            let receiver = vocal_isolation_node.connect_input_source(receiver);
            let receiver = concatenation_node.connect_input_source(receiver);
            let receiver = speech_recognition_node.connect_input_source(receiver);
            let receiver = text_translation_node.connect_input_source(receiver);
            let mut receiver = persistence_node.connect_input_source(receiver);
            let microphone_gain = app_state.get_microphone_gain();
            let speech_threshold = app_state.get_speech_threshold();
            let speech_merge_threshold = app_state.get_audio_gap_threshold();
            let silence_streak_count = app_state.get_silence_streak_count_lock();
            gain_control_node.set_gain(microphone_gain);
            vocal_isolation_node.set_silence_streak_count(silence_streak_count);
            vocal_isolation_node.set_speech_threshold(speech_threshold);
            concatenation_node.set_audio_gap_threshold(speech_merge_threshold);
            audio_context.connect_stream_input_node(stream_input_node);
            audio_context.connect_gain_control_node(gain_control_node);
            audio_context.connect_vad_node(vad_node);
            audio_context.connect_vocal_isolation_node(vocal_isolation_node);
            audio_context.connect_concatenation_node(concatenation_node);
            audio_context.connect_persistence_node(persistence_node);
            audio_context.connect_speech_recognition_node(speech_recognition_node);
            audio_context.connect_text_translation_node(text_translation_node);
            audio_context.start();
            tokio::spawn(async move {
                while let Some(result) = receiver.recv().await {
                    if let Err(err) = app.emit(
                        EVENT_NAME,
                        TranscriptData::new("".to_string(), result.speech_text().to_string()),
                    ) {
                        log_error!(
                            "Failed to send the transcript result to the frontend: {}",
                            err
                        );
                    }
                }
            });
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
pub fn stop_recording(app_state: State<'_, AppState>) -> Result<bool, String> {
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
