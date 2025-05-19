use crate::audio::context::node::concatenation::ConcatenationResult;
use crate::audio::context::node::Node;
use crate::audio::AudioBlock;
use crate::{log_info, log_warn};
use chrono::{DateTime, Local};
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use whisper_rs::{
    FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters, WhisperState,
};

pub struct TranscriptResult {
    pub start_record_time: DateTime<Local>,
    pub end_record_time: DateTime<Local>,
    pub speech_data: AudioBlock,
    pub speech_text: String,
}

impl TranscriptResult {
    pub fn new(
        start_record_time: DateTime<Local>,
        end_record_time: DateTime<Local>,
        speech_data: AudioBlock,
        speech_text: String,
    ) -> Self {
        TranscriptResult {
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

pub struct TranscriptNode {
    model_path: Arc<RwLock<PathBuf>>,
    sender: Option<Sender<TranscriptResult>>,
    input_source: Option<Receiver<ConcatenationResult>>,
    output_source: Option<Receiver<TranscriptResult>>,
}

impl TranscriptNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = channel::<TranscriptResult>(channel_capacity);
        let default_model_path = match env::current_dir() {
            Ok(path) => path.join("ggml-large-v3.bin"),
            Err(error) => panic!("Failed to get current directory： {}", error),
        };
        log_info!(
            "Speech recognition node created with default model path: {:?}",
            default_model_path
        );
        TranscriptNode {
            sender: Some(sender),
            model_path: Arc::new(RwLock::new(default_model_path)),
            input_source: None,
            output_source: Some(output_source),
        }
    }

    pub fn create_whisper_state(model_path: PathBuf) -> WhisperState {
        let model_path = model_path
            .to_str()
            .expect("The whisper model path cannot be converted to a UTF-8 string");
        let whisper_context =
            WhisperContext::new_with_params(model_path, WhisperContextParameters::default())
                .unwrap_or_else(|err| panic!("Failed to create whisper context: {}", err));
        whisper_context
            .create_state()
            .unwrap_or_else(|err| panic!("Failed to create whisper state: {}", err))
    }

    pub fn create_whisper_full_params() -> FullParams<'static, 'static> {
        let mut whisper_full_params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
        whisper_full_params.set_language(Some("auto"));
        let prompt = "请使用简体中文转录，并添加标点符号。例如：你好，mengen.dai，你今天怎么样？我很好，谢谢。不要使用繁体字。";
        whisper_full_params.set_initial_prompt(prompt);
        whisper_full_params
    }
}

impl Node<ConcatenationResult, TranscriptResult> for TranscriptNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<ConcatenationResult>,
    ) -> Receiver<TranscriptResult> {
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
                let model_path = model_path
                    .read()
                    .unwrap_or_else(|err| panic!("Failed to read whisper model path: {:#?}", err))
                    .clone();
                let mut whisper_state = TranscriptNode::create_whisper_state(model_path);
                let whisper_full_params = TranscriptNode::create_whisper_full_params();
                while let Some(result) = receiver.recv().await {
                    let speech_data = result.speech_data();
                    if let Err(err) = whisper_state.full(whisper_full_params.clone(), speech_data) {
                        log_warn!("Failed to run whisper model: {}", err);
                    }
                    let num_segments = whisper_state.full_n_segments().unwrap_or_else(|err| {
                        log_warn!("Failed to get number of segments: {}", err);
                        0
                    });
                    let mut speech_text = String::new();
                    for i in 0..num_segments {
                        let segment = whisper_state
                            .full_get_segment_text(i)
                            .expect("Failed to get segment text.");
                        speech_text.push_str(&segment);
                    }
                    let start_record_time = result.start_record_time();
                    let end_record_time = result.end_record_time();
                    let transcript_result = TranscriptResult::new(
                        start_record_time.clone(),
                        end_record_time.clone(),
                        speech_data.clone(),
                        speech_text,
                    );
                    if let Err(err) = sender
                        .as_ref()
                        .unwrap()
                        .send(transcript_result)
                        .await
                    {
                        log_warn!("Transcript node failed to send transcript result to the output source: {}", err);
                    }
                }
                log_info!("Transcript node has been stopped.");
            });
        }
    }

    fn deactivate(&mut self) {
        self.sender = None;
    }
}
