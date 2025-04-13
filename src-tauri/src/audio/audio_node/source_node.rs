use crate::audio::audio_node::AudioNode;
use crate::audio::AudioFrame;
use crate::log_error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct SourceNode {
    sender: Sender<AudioFrame>,
    input_source: Option<Receiver<AudioFrame>>,
    output_source: Option<Receiver<AudioFrame>>,
}

impl SourceNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = mpsc::channel::<AudioFrame>(channel_capacity);
        SourceNode {
            sender,
            input_source: None,
            output_source: Some(output_source),
        }
    }
}

impl AudioNode<AudioFrame, AudioFrame> for SourceNode {
    fn connect_input_source(&mut self, input_source: Receiver<AudioFrame>) -> Receiver<AudioFrame> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            log_error!("Source node output source is None");
            panic!("Source node output source is None")
        })
    }

    fn process(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let sender = self.sender.clone();
            tokio::spawn(async move {
                while let Some(samples) = receiver.recv().await {
                    if let Err(err) = sender.send(samples).await {
                        log_error!(
                            "Source node failed to send audio frame to receiver: {}",
                            err
                        );
                    }
                }
            });
        }
    }
}
