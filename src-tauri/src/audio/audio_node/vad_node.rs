use crate::audio::audio_node::AudioNode;
use crate::audio::AudioBlock;
use crate::log_error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use voice_activity_detector::VoiceActivityDetector;

pub struct VADResult {
    probability: f32,
    samples: AudioBlock,
}

impl VADResult {
    pub fn new(probability: f32, samples: AudioBlock) -> Self {
        VADResult {
            probability,
            samples,
        }
    }

    pub fn probability(&self) -> f32 {
        self.probability
    }

    pub fn samples(&self) -> &AudioBlock {
        &self.samples
    }

    #[allow(dead_code)]
    pub fn into_samples(self) -> AudioBlock {
        self.samples
    }
}

pub struct VadNode {
    chunk_size: usize,
    sample_rate: u32,
    sender: Sender<VADResult>,
    input_source: Option<Receiver<AudioBlock>>,
    output_source: Option<Receiver<VADResult>>,
}

impl VadNode {
    pub fn new(channel_capacity: usize, sample_rate: u32, chunk_size: usize) -> Self {
        let (sender, output_source) = mpsc::channel::<VADResult>(channel_capacity);
        VadNode {
            sender,
            chunk_size,
            sample_rate,
            input_source: None,
            output_source: Some(output_source),
        }
    }
}

impl AudioNode<AudioBlock, VADResult> for VadNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<AudioBlock>,
    ) -> Receiver<VADResult> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            log_error!("Vad node output source is None");
            panic!("Vad node output source is None")
        })
    }

    fn process(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let chunk_size = self.chunk_size;
            let sample_rate = self.sample_rate;
            let sender = self.sender.clone();
            let mut vad = match VoiceActivityDetector::builder()
                .sample_rate(sample_rate)
                .chunk_size(chunk_size)
                .build() // https://github.com/nkeenan38/voice_activity_detector
            {
                Ok(voice_activity_detector) => voice_activity_detector,
                Err(e) => {
                    log_error!("Failed to create VAD: {}", e);
                    panic!("Failed to create VAD: {}", e);
                }
            };
            tokio::spawn(async move {
                while let Some(samples) = receiver.recv().await {
                    let probability = vad.predict(samples.clone());
                    if let Err(err) = sender.send(VADResult::new(probability, samples)).await {
                        log_error!("Vad node failed to send audio frame to receiver: {}", err);
                    }
                }
            });
        }
    }
}
