use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use crate::audio::node::concatenation::ConcatenationResult;
use crate::audio::AudioBlock;
use chrono::{DateTime, Local};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
use crate::audio::node::AudioNode;
use crate::{log_error, log_info, log_warn};

pub struct SpeechRecognitionResult {
    pub start_record_time: DateTime<Local>,
    pub end_record_time: DateTime<Local>,
    pub speech_data: AudioBlock,
    pub speech_text: String,
}

impl SpeechRecognitionResult {
    pub fn new(
        start_record_time: DateTime<Local>,
        end_record_time: DateTime<Local>,
        speech_data: AudioBlock,
        speech_text: String,
    ) -> Self {
        SpeechRecognitionResult {
            start_record_time,
            end_record_time,
            speech_data,
            speech_text,
        }
    }

    pub fn start_record_time(&self) -> &DateTime<Local> {
        &self.start_record_time
    }

    #[allow(dead_code)]
    pub fn into_start_record_time(self) -> DateTime<Local> {
        self.start_record_time
    }

    #[allow(dead_code)]
    pub fn end_record_time(&self) -> &DateTime<Local> {
        &self.end_record_time
    }

    #[allow(dead_code)]
    pub fn into_end_record_time(self) -> DateTime<Local> {
        self.end_record_time
    }

    pub fn speech_data(&self) -> &AudioBlock {
        &self.speech_data
    }

    #[allow(dead_code)]
    pub fn into_speech_data(self) -> AudioBlock {
        self.speech_data
    }

    pub fn speech_text(&self) -> &str {
        &self.speech_text
    }

    #[allow(dead_code)]
    pub fn into_speech_text(self) -> String {
        self.speech_text
    }
}

pub struct SpeechRecognitionNode {
    model_path: Arc<RwLock<PathBuf>>,
    sender: Sender<SpeechRecognitionResult>,
    input_source: Option<Receiver<ConcatenationResult>>,
    output_source: Option<Receiver<SpeechRecognitionResult>>,
}

impl SpeechRecognitionNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = channel::<SpeechRecognitionResult>(channel_capacity);
        let default_model_path = match env::current_dir() {
            Ok(path) => path.join("ggml-large-v3.bin"),
            Err(error) => panic!("Failed to get current directory： {}", error),
        };
        log_info!("Speech recognition node created with default model path: {:?}", default_model_path);
        SpeechRecognitionNode {
            sender,
            model_path: Arc::new(RwLock::new(default_model_path)),
            input_source: None,
            output_source: Some(output_source),
        }
    }
}

impl AudioNode<ConcatenationResult, SpeechRecognitionResult> for SpeechRecognitionNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<ConcatenationResult>,
    ) -> Receiver<SpeechRecognitionResult> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            panic!(
                "Failed to take output source from speech recognition node: output source is None"
            )
        })
    }
    
    fn activate(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let sender = self.sender.clone();
            let model_path = self.model_path.clone();
            tokio::spawn(async move {
                let model_path = match model_path.read() {
                    Ok(model_path) => model_path.clone(),
                    Err(err) => {
                        log_error!("Failed to convert model path to string: {}", err);
                        panic!("Failed to read whisper model path: {}", err)
                    },
                };
                if let Some(model_path) = model_path.to_str() {
                    let context = match WhisperContext::new_with_params(model_path, WhisperContextParameters::default()) {
                        Ok(context) => context,
                        Err(err) => {
                            log_error!("Failed to create whisper context: {}", err);
                            panic!("Failed to create whisper context: {}", err)
                        },
                    };
                    let mut whisper_state = match context.create_state() {
                        Ok(state) => state,
                        Err(err) => {
                            log_error!("Failed to create whisper state: {}", err);
                            panic!("Failed to create whisper state: {}", err)
                        },
                    };
                    let whisper_full_params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
                    while let Some(result) = receiver.recv().await {
                        log_info!("Speech recognition node received concatenation result");
                        let speech_data = result.speech_data();
                        if let Err(err) = whisper_state.full(whisper_full_params.clone(), speech_data) {
                            log_error!("Failed to run whisper model: {}", err);
                        }
                        log_info!("whisper_state.full method end");
                        let num_segments = whisper_state.full_n_segments().unwrap_or_else(|err| {
                            log_error!("Failed to get number of segments: {}", err);
                            0
                        });
                        log_info!("num_segments: {}", num_segments);
                        let mut speech_text = String::new();
                        for i in 0..num_segments {
                            let segment = whisper_state.full_get_segment_text(i).expect("Failed to get segment text.");
                            speech_text.push_str(&segment);
                        }
                        log_info!("Speech recognition node received speech recognition result: {}", speech_text);
                        let start_record_time = result.start_record_time();
                        let end_record_time = result.end_record_time();
                        let speech_recognition_result = SpeechRecognitionResult::new(
                            start_record_time.clone(),
                            end_record_time.clone(),
                            speech_data.clone(),
                            speech_text,
                        );
                        if let Err(err) = sender.send(speech_recognition_result).await {
                            log_warn!("Speech recognition node failed to send speech recognition result to the output source: {}", err);
                        }
                    }
                } else {
                    panic!("The whisper model path cannot be converted to a UTF-8 string.");
                }
            });
        }
    }
}