use crate::audio::audio_node::speech_assembler_node::SpeechAssemblerResult;
use crate::audio::audio_node::AudioNode;
use crate::utils::wav_utils;
use std::env;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct PersistenceNode {
    sender: Sender<SpeechAssemblerResult>,
    input_source: Option<Receiver<SpeechAssemblerResult>>,
    output_source: Option<Receiver<SpeechAssemblerResult>>,
}

impl PersistenceNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = channel::<SpeechAssemblerResult>(channel_capacity);
        PersistenceNode {
            sender,
            input_source: None,
            output_source: Some(output_source),
        }
    }
}

impl AudioNode<SpeechAssemblerResult, SpeechAssemblerResult> for PersistenceNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<SpeechAssemblerResult>,
    ) -> Receiver<SpeechAssemblerResult> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            panic!("Wav writer node output source is None");
        })
    }

    fn process(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let sender = self.sender.clone();
            tokio::spawn(async move {
                let tmp_dir = env::temp_dir().join("talk-bridge-live");
                while let Some(result) = receiver.recv().await {
                    let samples = result.samples();
                    let start_record_time = result.start_record_time();
                    let file_name =
                        format!("{}.wav", start_record_time.format("%Y-%m-%d_%H-%M-%S.%3f"));
                    let file_path = tmp_dir.join(file_name);
                    wav_utils::save_wav_file_async(file_path, samples);
                    if let Err(err) = sender.send(result).await {
                        log::error!("Wav writer node failed to send result to receiver: {}", err);
                    }
                }
            });
        }
    }
}
