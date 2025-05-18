use crate::audio::context::node::Node;
use crate::audio::AudioBlock;
use crate::{log_info, log_warn};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use voice_activity_detector::VoiceActivityDetector;

pub struct VoiceActivityDetectionResult {
    probability: f32,
    samples: AudioBlock,
}

impl VoiceActivityDetectionResult {
    pub fn new(probability: f32, samples: AudioBlock) -> Self {
        VoiceActivityDetectionResult {
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

pub struct VoiceActivityDetectionNode {
    chunk_size: usize,
    sample_rate: u32,
    sender: Option<Sender<VoiceActivityDetectionResult>>,
    input_source: Option<Receiver<AudioBlock>>,
    output_source: Option<Receiver<VoiceActivityDetectionResult>>,
}

impl VoiceActivityDetectionNode {
    pub fn new(channel_capacity: usize, sample_rate: u32, chunk_size: usize) -> Self {
        let (sender, output_source) =
            mpsc::channel::<VoiceActivityDetectionResult>(channel_capacity);
        VoiceActivityDetectionNode {
            sender: Some(sender),
            chunk_size,
            sample_rate,
            input_source: None,
            output_source: Some(output_source),
        }
    }
}

impl Node<AudioBlock, VoiceActivityDetectionResult> for VoiceActivityDetectionNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<AudioBlock>,
    ) -> Receiver<VoiceActivityDetectionResult> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            panic!("Failed to take output source from voice activity detection node: output source is None")
        })
    }

    fn activate(&mut self) {
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
                Err(error) => {
                    panic!("Failed to create voice activity detector: {}", error);
                }
            };
            tokio::spawn(async move {
                while let Some(samples) = receiver.recv().await {
                    let probability = vad.predict(samples.clone());
                    if let Err(err) = sender.as_ref().unwrap()
                        .send(VoiceActivityDetectionResult::new(probability, samples))
                        .await
                    {
                        log_warn!("Voice activity detection node failed to send audio data to the output source: {}", err);
                    }
                }
                log_info!("Voice activity detection node has been stopped.");
            });
        }
    }
    
    fn deactivate(&mut self) {
        self.sender = None;
    }
}
