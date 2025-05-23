use crate::app_state::DEFAULT_MICROPHONE_GAIN;
use crate::audio::context::node::Node;
use crate::audio::AudioBlock;
use crate::{log_info, log_warn};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct GainControlNode {
    gain: Arc<RwLock<f32>>,
    sender: Option<Sender<AudioBlock>>,
    input_source: Option<Receiver<AudioBlock>>,
    output_source: Option<Receiver<AudioBlock>>,
}

impl GainControlNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = mpsc::channel::<AudioBlock>(channel_capacity);
        GainControlNode {
            gain: Arc::new(RwLock::new(DEFAULT_MICROPHONE_GAIN)),
            sender: Some(sender),
            input_source: None,
            output_source: Some(output_source),
        }
    }

    pub fn set_gain(&mut self, gain: Arc<RwLock<f32>>) {
        self.gain = gain;
    }
}

impl Node<AudioBlock, AudioBlock> for GainControlNode {
    fn connect_input_source(&mut self, input_source: Receiver<AudioBlock>) -> Receiver<AudioBlock> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            panic!("Failed to take output source from gain control node: output source is None")
        })
    }

    fn activate(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let gain = self.gain.clone();
            let sender = self.sender.clone();
            tokio::spawn(async move {
                while let Some(samples) = receiver.recv().await {
                    let gain = gain.read().map_or(DEFAULT_MICROPHONE_GAIN, |gain| *gain);
                    let samples = samples.iter().map(|&x| x * gain).collect();
                    if let Err(err) = sender.as_ref().unwrap().send(samples).await {
                        log_warn!("Gain control node failed to send audio data to the output source: {}", err);
                    }
                }
                log_info!("Gain control node has been stopped.");
            });
        }
    }

    fn deactivate(&mut self) {
        self.sender = None;
    }
}
