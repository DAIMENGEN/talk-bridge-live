use crate::log_error;
use crate::protos_gen::protos::chat_service_client::ChatServiceClient;
use crate::protos_gen::protos::{ChatRequest, MeetingRoom};
use tonic::transport::Channel;
use tonic::Request;

pub struct SpeechTranslatorClient {
    client: ChatServiceClient<Channel>,
}

impl SpeechTranslatorClient {
    pub async fn new(grpc_address: String) -> Self {
        let client = ChatServiceClient::connect(grpc_address).await.unwrap();
        SpeechTranslatorClient { client }
    }

    pub async fn send(&mut self) {
        let request = Request::new(ChatRequest {
            speaker: "".to_string(),
            meeting_room: "".to_string(),
            start: 0,
            end: 0,
            sample_rate: 16000,
            audio_bytes: vec![],
            target_language: vec!["cmn".to_string(), "eng".to_string(), "jpn".to_string()],
            tag: "".to_string(),
            tag64: 1,
        });
        if let Err(err) = self.client.send(request).await {
            log_error!(
                "Failed to send request: {} | speaker: {} | meeting_room: {}",
                err,
                "speaker",
                "meeting_room"
            );
        }
    }

    pub async fn receive(&mut self) {
        let request = Request::new(MeetingRoom {
            meeting_room: "".to_string(),
            password: "".to_string()
        });
        match self.client.chat_listen(request).await {
            Ok(response) => {
                let mut stream = response.into_inner();
                stream.message().for_each(|item| {
                    println!("Received: {:?}", item);
                });
            }
            Err(err) => {
                log_error!("Failed to receive response: {}", err);
            }
        }
    }
}
