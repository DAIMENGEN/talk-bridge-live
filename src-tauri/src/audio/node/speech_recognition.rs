use crate::audio::node::concatenation::ConcatenationResult;
use crate::audio::AudioBlock;
use chrono::{DateTime, Local};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use crate::audio::node::AudioNode;
use crate::log_warn;

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
    sender: Sender<SpeechRecognitionResult>,
    input_source: Option<Receiver<ConcatenationResult>>,
    output_source: Option<Receiver<SpeechRecognitionResult>>,
}

impl SpeechRecognitionNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = channel::<SpeechRecognitionResult>(channel_capacity);
        SpeechRecognitionNode {
            sender,
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
            tokio::spawn(async move {
                while let Some(result) = receiver.recv().await {
                    let speech_data = result.speech_data();
                    let speech_text = "Speech text".to_string();
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
            });
        }
    }
}