use crate::audio::audio_context::AudioContext;
use crate::audio::audio_node::AudioNode;
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
            let mut source_node = audio_context.create_source_node();
            let mut gain_node = audio_context.create_gain_node();
            let mut vad_node = audio_context.create_vad_node();
            let mut speech_extractor_node = audio_context.create_speech_extractor_node();
            let mut speech_assembler_node = audio_context.create_speech_assembler_node();
            let mut wav_writer_node = audio_context.create_wav_writer_node();
            let asr_service_url = app_state.get_asr_service_url();
            let asr_service_url = asr_service_url
                .read()
                .map_err(|err| format!("Failed to read ASR service URL: {}", err))?
                .clone();
            let mut speech_translator_node =
                audio_context.create_speech_translator_node(asr_service_url);
            let receiver = source_node.connect_input_source(receiver);
            let receiver = gain_node.connect_input_source(receiver);
            let receiver = vad_node.connect_input_source(receiver);
            let receiver = speech_extractor_node.connect_input_source(receiver);
            let receiver = speech_assembler_node.connect_input_source(receiver);
            let receiver = wav_writer_node.connect_input_source(receiver);
            let mut receiver = speech_translator_node.connect_input_source(receiver);
            let speaker = app_state.get_speaker();
            let tolerance = app_state.get_audio_tolerance();
            let meeting_room = app_state.get_meeting_room();
            let microphone_gain = app_state.get_microphone_gain();
            let speech_threshold = app_state.get_speech_threshold();
            let speech_merge_threshold = app_state.get_speech_merge_threshold();
            gain_node.set_gain(microphone_gain);
            speech_extractor_node.set_tolerance(tolerance);
            speech_extractor_node.set_speech_threshold(speech_threshold);
            speech_assembler_node.set_merge_threshold(speech_merge_threshold);
            speech_translator_node.set_speaker(speaker);
            speech_translator_node.set_meeting_room(meeting_room);
            audio_context.connect_source_node(source_node);
            audio_context.connect_gain_node(gain_node);
            audio_context.connect_vad_node(vad_node);
            audio_context.connect_speech_extractor_node(speech_extractor_node);
            audio_context.connect_speech_assembler_node(speech_assembler_node);
            audio_context.connect_wav_writer_node(wav_writer_node);
            audio_context.connect_speech_translator_node(speech_translator_node);
            audio_context.start();
            tokio::spawn(async move {
                while let Some(result) = receiver.recv().await {
                    if let Err(err) = app.emit(
                        EVENT_NAME,
                        TranscriptData::new("".to_string(), result.chinese_text().to_string()),
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
