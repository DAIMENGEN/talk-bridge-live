use crate::audio::audio_node::speech_assembler_node::SpeechAssemblerResult;
use crate::audio::audio_node::AudioNode;
use crate::log_error;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct SpeechTranslatorResult {
    text: String,
}

impl SpeechTranslatorResult {
    pub fn new(text: String) -> Self {
        SpeechTranslatorResult { text }
    }

    #[allow(dead_code)]
    pub fn text(&self) -> &String {
        &self.text
    }

    #[allow(dead_code)]
    pub fn into_text(self) -> String {
        self.text
    }
}

pub struct SpeechTranslatorNode {
    speaker: Arc<RwLock<String>>,
    meeting_room: Arc<RwLock<String>>,
    sender: Sender<SpeechTranslatorResult>,
    input_source: Option<Receiver<SpeechAssemblerResult>>,
    output_source: Option<Receiver<SpeechTranslatorResult>>,
}

impl SpeechTranslatorNode {
    pub fn new(channel_capacity: usize) -> Self {
        let username = whoami::username();
        let hostname = whoami::fallible::hostname().unwrap_or(username.clone());
        let (sender, output_source) = channel::<SpeechTranslatorResult>(channel_capacity);
        SpeechTranslatorNode {
            speaker: Arc::new(RwLock::new(username)),
            meeting_room: Arc::new(RwLock::new(hostname)),
            sender,
            input_source: None,
            output_source: Some(output_source),
        }
    }

    #[allow(dead_code)]
    pub fn set_speaker(&mut self, speaker: Arc<RwLock<String>>) {
        self.speaker = speaker;
    }

    #[allow(dead_code)]
    pub fn set_meeting_room(&mut self, meeting_room: Arc<RwLock<String>>) {
        self.meeting_room = meeting_room;
    }
}

impl AudioNode<SpeechAssemblerResult, SpeechTranslatorResult> for SpeechTranslatorNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<SpeechAssemblerResult>,
    ) -> Receiver<SpeechTranslatorResult> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            log_error!("Speech translator node output source is None");
            panic!("Speech translator node output source is None")
        })
    }

    fn process(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let sender = self.sender.clone();
            tokio::spawn(async move {
                while let Some(result) = receiver.recv().await {
                    let transcript = format!(
                        "{} {} {}",
                        result
                            .start_record_time()
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string(),
                        result
                            .end_record_time()
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string(),
                        result.samples().len()
                    );
                    if let Err(err) = sender.send(SpeechTranslatorResult::new(transcript)).await {
                        log_error!(
                            "Speech translator node failed to send speech translator result to receiver: {}",
                            err
                        );
                    }
                }
            });
        }
    }
}
