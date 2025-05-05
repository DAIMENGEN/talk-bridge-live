use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use crate::log_error;
use crate::protos_gen::protos::chat_service_client::ChatServiceClient;
use crate::protos_gen::protos::{ChatRequest, ChatRespond, MeetingRoom};
use tonic::transport::Channel;
use tonic::Request;

pub struct SpeechTranslatorClient {
    client: ChatServiceClient<Channel>,
}

impl SpeechTranslatorClient {
    pub async fn new(asr_service_url: String) -> Self {
        let client = ChatServiceClient::connect(asr_service_url).await.unwrap();
        SpeechTranslatorClient { client }
    }

    pub async fn send(&mut self, speaker: String,  meeting_room: String, sample_rate: usize, audio_bytes: Vec<u8>) {
        let request = Request::new(ChatRequest {
            speaker: speaker.clone(),
            meeting_room: meeting_room.clone(),
            start: 0,
            end: 0,
            sample_rate: sample_rate as i64,
            audio_bytes,
            target_language: vec!["cmn".to_string(), "eng".to_string(), "jpn".to_string()],
            tag: "".to_string(),
            tag64: 1,
        });
        if let Err(err) = self.client.chat_send(request).await {
            log_error!(
                "Failed to send request: {} | speaker: {} | meeting_room: {}",
                err,
                speaker,
                meeting_room
            );
        }
    }

    pub async fn receive(&mut self, meeting_room: String, password: String) -> Receiver<ChatRespond>{
        let (sender, receiver) = mpsc::channel::<ChatRespond>(100);
        let request = Request::new(MeetingRoom {
            meeting_room,
            password
        });
        match self.client.chat_listen(request).await {
            Ok(response) => {
                let mut stream = response.into_inner();
                tokio::spawn(async move {
                    while let Some(result) = stream.message().await.transpose() {
                        match result {
                            Ok(response) => {
                                if let Err(err) = sender.send(response).await {
                                    log_error!("Failed to send translator result response: {}", err);
                                }
                            }
                            Err(err) => {
                                log_error!("Failed to receive translator result response: {}", err);
                            }
                        }
                    }
                });
                receiver
            }
            Err(err) => {
                log_error!("Failed to listen translator result response: {}", err);
                panic!("Failed to listen translator result response");
            }
        }
    }
}
