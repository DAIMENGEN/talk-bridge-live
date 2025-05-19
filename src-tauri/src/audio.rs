use crate::app_state::AppState;
use crate::audio::context::microphone_context::MicrophoneContext;
use crate::audio::context::node::Node;
use crate::device::device_manager::get_microphone_by_name;
use crate::device::input::microphone::Microphone;
use crate::log_error;
use crate::utils::time_utils::format_local_datetime;
use serde::Serialize;
use std::collections::HashMap;
use tauri::{AppHandle, Emitter, State};

pub mod context;
pub type AudioSample = f32;
pub type AudioBlock = Vec<AudioSample>;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Utterance {
    datetime: String,
    speech_data: AudioBlock,
    speech_text: String,
    translated_texts: HashMap<String, String>,
}

#[tauri::command(rename_all = "snake_case")]
pub fn start_asr(
    app_state: State<'_, AppState>,
    app_handle: AppHandle,
    microphone_name: String,
) -> Result<String, String> {
    match get_microphone_by_name(&microphone_name) {
        Ok(device) => {
            const EVENT_NAME: &str = "transcript_result_event";
            let microphone = Microphone::new(device);
            let mut microphone_context = MicrophoneContext::new(microphone);
            let receiver = microphone_context.init().unwrap();
            let mut stream_input_node = microphone_context.create_stream_input_node();
            let mut gain_control_node = microphone_context.create_gain_control_node();
            let mut vad_node = microphone_context.create_vad_node();
            let mut vocal_isolation_node = microphone_context.create_vocal_isolation_node();
            let mut concatenation_node = microphone_context.create_concatenation_node();
            let mut persistence_node = microphone_context.create_persistence_node();
            let mut transcript_node = microphone_context.create_transcript_node();
            let mut translation_node = microphone_context.create_translation_node();
            let receiver = stream_input_node.connect_input_source(receiver);
            let receiver = gain_control_node.connect_input_source(receiver);
            let receiver = vad_node.connect_input_source(receiver);
            let receiver = vocal_isolation_node.connect_input_source(receiver);
            let receiver = concatenation_node.connect_input_source(receiver);
            let receiver = transcript_node.connect_input_source(receiver);
            let receiver = translation_node.connect_input_source(receiver);
            let mut receiver = persistence_node.connect_input_source(receiver);
            let microphone_gain = app_state.get_microphone_gain();
            let speech_threshold = app_state.get_speech_threshold();
            let silence_streak_threshold = app_state.get_silence_streak_threshold();
            gain_control_node.set_gain(microphone_gain);
            vocal_isolation_node.set_silence_streak_threshold(silence_streak_threshold);
            vocal_isolation_node.set_speech_threshold(speech_threshold);
            microphone_context.connect_stream_input_node(stream_input_node);
            microphone_context.connect_gain_control_node(gain_control_node);
            microphone_context.connect_vad_node(vad_node);
            microphone_context.connect_vocal_isolation_node(vocal_isolation_node);
            microphone_context.connect_concatenation_node(concatenation_node);
            microphone_context.connect_persistence_node(persistence_node);
            microphone_context.connect_transcript_node(transcript_node);
            microphone_context.connect_translation_node(translation_node);
            microphone_context.start();
            tokio::spawn(async move {
                while let Some(result) = receiver.recv().await {
                    let speech_text = result.speech_text().to_owned();
                    let translated_texts = result.translated_texts().to_owned();
                    if let Err(err) = app_handle.emit(
                        EVENT_NAME,
                        Utterance {
                            datetime: format_local_datetime(result.start_record_time),
                            speech_text,
                            translated_texts,
                            speech_data: result.into_speech_data(),
                        },
                    ) {
                        log_error!(
                            "Failed to send the transcript result to the frontend: {}",
                            err
                        );
                    }
                }
            });
            match app_state.set_microphone_context(microphone_context) {
                Ok(_) => Ok(EVENT_NAME.parse().unwrap()),
                Err(err) => Err(format!("Failed to save recording context: {}", err)),
            }
        }
        Err(err) => Err(format!(
            "Microphone {} not found, error: {}",
            microphone_name, err
        )),
    }
}

#[tauri::command]
pub fn stop_asr(app_state: State<'_, AppState>) -> Result<bool, String> {
    let microphone_context = app_state.get_microphone_context();
    let mut microphone_context_lock = microphone_context
        .lock()
        .map_err(|err| format!("Failed to lock microphone: {}", err))?;
    // Here, ownership of context is taken from app_state using take.
    // Once it's taken, app_state no longer owns context, and everything related to context will be dropped and cleaned up.
    if let Some(mut context) = microphone_context_lock.take() {
        context.close();
    }
    Ok(true)
}

// 音频帧：每一帧包含了一组样本，通常与音频的通道数量（例如立体声是2个通道）相关联。例如，在立体声的情况下，每一帧包含两个样本：一个代表左声道，另一个代表右声道。如果是单声道，则每一帧只有一个样本。
// 音频样本：表示音频信号的单一数据点，通常是一个浮动的值，代表某一时刻的音频信号强度。每个样本通常对应于特定的时间点。
// 音频块：在某些情况下，多个音频帧会被聚集成一个更大的数据块，这个数据块可以在处理时作为一个整体进行操作。通常这些块的大小是根据一定的时间长度或者其他特定需求来确定的。
// 话语、发生：Utterance（话语、发声） 指的是一次完整的语言表达，通常是一个人在某一时刻说出的一句话、短语或一段话，以一次开口到停顿为界限。它不一定是语法完整的句子，但在语音或语义上是一个独立单位。

// Audio Frame: An audio frame contains a group of samples, typically associated with the number of audio channels (for example, 2 channels for stereo).
// In the case of stereo, each frame contains two samples: one for the left channel and one for the right channel. If the audio is mono, each frame contains only a single sample.

// Audio Sample: An audio sample represents a single data point of the audio signal, usually a floating-point value that reflects the amplitude of the sound at a specific point in time.
// Each sample corresponds to a precise moment in the audio stream.

// Audio Block: In some contexts, multiple audio frames are grouped together into a larger data block, which can be processed as a whole.
// The size of these blocks is usually determined based on a specific time duration or other processing requirements.

// Utterance: Utterance refers to a unit of speech or spoken language. It is any stretch of speech, spoken by a person, that may consist of a word, phrase, sentence, or even multiple sentences, typically bounded by silence or a pause.
