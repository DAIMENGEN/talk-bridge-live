use crate::audio::audio_node::speech_assembler_node::SpeechAssemblerResult;
use crate::audio::audio_node::AudioNode;
use crate::log_error;
use crate::protos_gen::protos::chat_service_client::ChatServiceClient;
use crate::protos_gen::protos::{ChatRequest, ChatRespond, MeetingRoom};
use crate::utils::wav_utils::encode_to_wav_bytes;
use chrono::{DateTime, Local};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tonic::Request;
use crate::language::Language;

pub struct SpeechTranslatorResult {
    speaker: String,
    datetime: DateTime<Local>,
    chinese_text: String,
    english_text: String,
    japanese_text: String,
    german_text: String,
    spanish_text: String,
    korean_text: String
}

impl SpeechTranslatorResult {
    pub fn new(response: ChatRespond) -> Self {
        let datetime = DateTime::from_timestamp(response.start, 0).unwrap();
        let datetime = datetime.with_timezone(&Local);
        Self {
            speaker: response.speaker,
            datetime,
            chinese_text: "".to_string(),
            english_text: "".to_string(),
            japanese_text: "".to_string(),
            german_text: "".to_string(),
            spanish_text: "".to_string(),
            korean_text: "".to_string()
        }
    }

    #[allow(dead_code)]
    pub fn speaker(&self) -> &str {
        &self.speaker
    }

    #[allow(dead_code)]
    pub fn datetime(&self) -> &DateTime<Local> {
        &self.datetime
    }

    #[allow(dead_code)]
    pub fn chinese_text(&self) -> &str {
        &self.chinese_text
    }

    #[allow(dead_code)]
    pub fn english_text(&self) -> &str {
        &self.english_text
    }

    #[allow(dead_code)]
    pub fn japanese_text(&self) -> &str {
        &self.japanese_text
    }

    #[allow(dead_code)]
    pub fn german_text(&self) -> &str {
        &self.german_text
    }

    #[allow(dead_code)]
    pub fn spanish_text(&self) -> &str {
        &self.spanish_text
    }

    #[allow(dead_code)]
    pub fn korean_text(&self) -> &str {
        &self.korean_text
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

    async fn send_request(
        asr_service_url: String,
        speaker: String,
        meeting_room: String,
        sample_rate: usize,
        audio_bytes: Vec<u8>,
    ) {
        if let Ok(mut translator_client) = ChatServiceClient::connect(asr_service_url).await {
            let request = Request::new(ChatRequest {
                speaker: speaker.clone(),
                meeting_room: meeting_room.clone(),
                start: 0,
                end: 0,
                sample_rate: sample_rate as i64,
                audio_bytes,
                target_language: vec![Language::Chinese.code().to_string(), Language::English.code().to_string(), Language::Japanese.code().to_string(), Language::German.code().to_string()],
                tag: "".to_string(),
                tag64: 1,
            });
            if let Err(err) = translator_client.chat_send(request).await {
                log_error!(
                    "Failed to send request: {} | speaker: {} | meeting_room: {}",
                    err,
                    speaker,
                    meeting_room
                );
            }
        }
    }

    async fn receive_response(
        asr_service_url: String,
        meeting_room: String,
        password: String,
        forward: Sender<SpeechTranslatorResult>,
    ) {
        if let Ok(mut translator_client) = ChatServiceClient::connect(asr_service_url).await {
            let request = Request::new(MeetingRoom {
                meeting_room,
                password
            });
            match translator_client.chat_listen(request).await {
                Ok(response) => {
                    let mut stream = response.into_inner();
                    while let Some(result) = stream.message().await.transpose() {
                        match result {
                            Ok(response) => {
                                if let Err(err) = forward.send(SpeechTranslatorResult::new(response)).await {
                                    log_error!("Failed to forward translator result response: {}", err);
                                }
                            }
                            Err(err) => {
                                log_error!("Failed to receive translator result response: {}", err);
                            }
                        };
                    }
                }
                Err(err) => {
                    log_error!("Failed to listen translator result response: {}", err);
                }
            };
        }
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
            let speaker = self.speaker.clone();
            let meeting_room = self.meeting_room.clone();
            tokio::spawn(async move {
                while let Some(result) = receiver.recv().await {
                    let address = asr_service_url.clone();
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
                    SpeechTranslatorNode::send_request(address, speaker, meeting_room, 16000, audio_bytes).await;
                }
            });
            let asr_service_url = self.asr_service_url.clone();
            let sender = self.sender.clone();
            let meeting_room = self.meeting_room.clone();
            tokio::spawn(async move {
                let password = "".to_string();
                let meeting_room = match meeting_room.read() {
                    Ok(meeting_room) => meeting_room.clone(),
                    Err(err) => {
                        panic!("Failed to read meeting room: {}", err);
                    }
                };
                SpeechTranslatorNode::receive_response(asr_service_url, meeting_room, password, sender).await;
            });
        }
    }
}
