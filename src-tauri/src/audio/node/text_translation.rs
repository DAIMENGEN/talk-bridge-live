use crate::audio::node::speech_recognition::SpeechRecognitionResult;
use crate::audio::node::AudioNode;
use crate::audio::AudioBlock;
use crate::log_warn;
use chrono::{DateTime, Local};
use std::collections::HashMap;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct TextTranslationResult {
    pub start_record_time: DateTime<Local>,
    pub end_record_time: DateTime<Local>,
    pub speech_data: AudioBlock,
    pub speech_text: String,
    pub translated_texts: HashMap<String, String>, // key is language code, value is translated text
}

impl TextTranslationResult {
    pub fn new(
        start_record_time: DateTime<Local>,
        end_record_time: DateTime<Local>,
        speech_data: AudioBlock,
        speech_text: String,
        translated_texts: HashMap<String, String>,
    ) -> Self {
        TextTranslationResult {
            start_record_time,
            end_record_time,
            speech_data,
            speech_text,
            translated_texts,
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

    pub fn translated_texts(&self) -> &HashMap<String, String> {
        &self.translated_texts
    }

    #[allow(dead_code)]
    pub fn into_translated_texts(self) -> HashMap<String, String> {
        self.translated_texts
    }
}

pub struct TextTranslationNode {
    sender: Option<Sender<TextTranslationResult>>,
    input_source: Option<Receiver<SpeechRecognitionResult>>,
    output_source: Option<Receiver<TextTranslationResult>>,
}

impl TextTranslationNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = channel::<TextTranslationResult>(channel_capacity);
        TextTranslationNode {
            sender: Some(sender),
            input_source: None,
            output_source: Some(output_source),
        }
    }
}

impl AudioNode<SpeechRecognitionResult, TextTranslationResult> for TextTranslationNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<SpeechRecognitionResult>,
    ) -> Receiver<TextTranslationResult> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            panic!("Failed to take output source from text translation node: output source is None")
        })
    }

    fn activate(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let sender = self.sender.clone();
            tokio::spawn(async move {
                while let Some(result) = receiver.recv().await {
                    let speech_data = result.speech_data();
                    let speech_text = result.speech_text();
                    let start_record_time = result.start_record_time();
                    let end_record_time = result.end_record_time();
                    let translated_texts = HashMap::new();
                    let text_translation_result = TextTranslationResult::new(
                        start_record_time.clone(),
                        end_record_time.clone(),
                        speech_data.clone(),
                        speech_text.to_string(),
                        translated_texts,
                    );
                    if let Err(err) = sender.as_ref().unwrap().send(text_translation_result).await {
                        log_warn!("Text translation node failed to send text translation result to the output source: {}", err);
                    }
                }
            });
        }
    }

    fn deactivate(&mut self) {
        self.sender = None;
    }
}
