use crate::audio::audio_node::speech_assembler_node::SpeechAssemblerResult;
use crate::audio::audio_node::AudioNode;
use crate::grpc_client::speech_translator_client::SpeechTranslatorClient;
use crate::log_error;
use crate::utils::wav_utils::encode_to_wav_bytes;
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
    asr_service_url: String,
    sender: Sender<SpeechTranslatorResult>,
    input_source: Option<Receiver<SpeechAssemblerResult>>,
    output_source: Option<Receiver<SpeechTranslatorResult>>,
}

impl SpeechTranslatorNode {
    pub fn new(channel_capacity: usize, asr_service_url: String) -> Self {
        let username = whoami::username();
        let hostname = whoami::fallible::hostname().unwrap_or(username.clone());
        let (sender, output_source) = channel::<SpeechTranslatorResult>(channel_capacity);
        SpeechTranslatorNode {
            speaker: Arc::new(RwLock::new(username)),
            meeting_room: Arc::new(RwLock::new(hostname)),
            asr_service_url,
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
            let asr_service_url = self.asr_service_url.clone();
            let sender = self.sender.clone();
            let speaker = self.speaker.clone();
            let meeting_room = self.meeting_room.clone();
            tokio::spawn(async move {
                let mut speech_translator_client = SpeechTranslatorClient::new(asr_service_url).await;
                while let Some(result) = receiver.recv().await {
                    let speaker = match speaker.read() { 
                        Ok(speaker) => speaker.clone(),
                        Err(err) => {
                            panic!("Failed to read speaker: {}", err);
                        }
                    };
                    let meeting_room = match meeting_room.read() {
                        Ok(meeting_room) => meeting_room.clone(),
                        Err(err) => {
                            panic!("Failed to read meeting room: {}", err);
                        }
                    };
                    let audio_bytes = encode_to_wav_bytes(result.samples());
                    speech_translator_client.send(speaker, meeting_room, 16000, audio_bytes).await;
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
