use crate::audio::node::AudioNode;
use crate::utils::wav_utils;
use std::env;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use crate::audio::node::text_translation::TextTranslationResult;
use crate::{log_info, log_warn};

pub struct PersistenceNode {
    sender: Option<Sender<TextTranslationResult>>,
    input_source: Option<Receiver<TextTranslationResult>>,
    output_source: Option<Receiver<TextTranslationResult>>,
}

impl PersistenceNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = channel::<TextTranslationResult>(channel_capacity);
        PersistenceNode {
            sender: Some(sender),
            input_source: None,
            output_source: Some(output_source),
        }
    }
}

impl AudioNode<TextTranslationResult, TextTranslationResult> for PersistenceNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<TextTranslationResult>,
    ) -> Receiver<TextTranslationResult> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            panic!("Failed to take output source from persistence node: output source is None")
        })
    }

    fn activate(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let sender = self.sender.clone();
            tokio::spawn(async move {
                let tmp_dir = env::temp_dir().join("talk-bridge-live");
                while let Some(result) = receiver.recv().await {
                    let samples = result.speech_data();
                    let start_record_time = result.start_record_time();
                    let file_name =
                        format!("{}.wav", start_record_time.format("%Y-%m-%d_%H-%M-%S.%3f"));
                    let file_path = tmp_dir.join(file_name);
                    wav_utils::save_wav_file_async(file_path, samples);
                    if let Err(err) = sender.as_ref().unwrap().send(result).await {
                        log_warn!("Persistence node failed to send text translation result to the output source: {}", err);
                    }
                }
                log_info!("Persistence node has been stopped.");
            });
        }
    }
    
    fn deactivate(&mut self) {
        self.sender = None;
    }
}
