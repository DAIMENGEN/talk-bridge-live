use crate::audio::node::AudioNode;
use crate::audio::AudioFrame;
use crate::{log_error, log_info};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct GainNode {
    gain: Arc<Mutex<f32>>,
    sender: Sender<AudioFrame>,
    input_source: Option<Receiver<AudioFrame>>,
    output_source: Option<Receiver<AudioFrame>>,
}

impl GainNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = mpsc::channel::<AudioFrame>(channel_capacity);
        GainNode {
            gain: Arc::new(Mutex::new(1.0)),
            sender,
            input_source: None,
            output_source: Some(output_source),
        }
    }

    pub fn set_gain(&mut self, gain: Arc<Mutex<f32>>) {
        self.gain = gain;
    }
}

impl AudioNode<AudioFrame> for GainNode {
    fn connect_input_source(&mut self, input_source: Receiver<AudioFrame>) -> Receiver<AudioFrame> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            log_error!("Gain node output source is None");
            panic!("Gain node output source is None")
        })
    }

    fn process(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let gain = self.gain.clone();
            let sender = self.sender.clone();
            tokio::spawn(async move {
                while let Some(samples) = receiver.recv().await {
                    let gain = if let Ok(gain) = gain.lock() {
                        *gain
                    } else {
                        1.0
                    };
                    let samples = samples.iter().map(|&x| x * gain).collect();
                    if let Err(err) = sender.send(samples).await {
                        log_error!("Gain node failed to send audio frame to receiver: {}", err);
                    }
                }
            });
        }
    }
}
