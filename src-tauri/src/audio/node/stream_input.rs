use crate::audio::node::AudioNode;
use crate::audio::AudioBlock;
use crate::log_warn;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct StreamInputNode {
    sender: Sender<AudioBlock>,
    input_source: Option<Receiver<AudioBlock>>,
    output_source: Option<Receiver<AudioBlock>>,
}

impl StreamInputNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = mpsc::channel::<AudioBlock>(channel_capacity);
        StreamInputNode {
            sender,
            input_source: None,
            output_source: Some(output_source),
        }
    }
}

impl AudioNode<AudioBlock, AudioBlock> for StreamInputNode {
    fn connect_input_source(&mut self, input_source: Receiver<AudioBlock>) -> Receiver<AudioBlock> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            panic!("Failed to take output source from stream input node: output source is None")
        })
    }

    fn activate(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let sender = self.sender.clone();
            tokio::spawn(async move {
                while let Some(samples) = receiver.recv().await {
                    if let Err(err) = sender.send(samples).await {
                        log_warn!(
                            "Stream input node failed to send audio data to the output source: {}",
                            err
                        );
                    }
                }
                receiver.close();
                sender.closed().await;
            });
        }
    }
}
