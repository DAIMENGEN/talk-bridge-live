use crate::audio::audio_node::vad_node::VadAudioFrame;
use crate::audio::audio_node::AudioNode;
use crate::audio::AudioFrame;
use crate::{log_error, log_warn};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::app_state::{DEFAULT_SPEECH_THRESHOLD, DEFAULT_TOLERANCE};



pub struct AssemblerNode {
    audio_tolerance: Arc<RwLock<usize>>,
    speech_threshold: Arc<RwLock<f32>>,
    sender: Sender<AudioFrame>,
    input_source: Option<Receiver<VadAudioFrame>>,
    output_source: Option<Receiver<AudioFrame>>,
}

impl AssemblerNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = mpsc::channel::<AudioFrame>(channel_capacity);
        AssemblerNode {
            audio_tolerance: Arc::new(RwLock::new(DEFAULT_TOLERANCE)),
            speech_threshold: Arc::new(RwLock::new(DEFAULT_SPEECH_THRESHOLD)),
            sender,
            input_source: None,
            output_source: Some(output_source),
        }
    }

    pub fn set_tolerance(&mut self, audio_tolerance: Arc<RwLock<usize>>) {
        self.audio_tolerance = audio_tolerance;
    }

    pub fn set_speech_threshold(&mut self, speech_threshold: Arc<RwLock<f32>>) {
        self.speech_threshold = speech_threshold;
    }
}

impl AudioNode<VadAudioFrame, AudioFrame> for AssemblerNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<VadAudioFrame>,
    ) -> Receiver<AudioFrame> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            log_error!("Assembler node output source is None");
            panic!("Assembler node output source is None")
        })
    }

    fn process(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let sender = self.sender.clone();
            let mut speech_frame = VecDeque::<f32>::new();
            let mut probabilities = VecDeque::<f32>::new();
            let audio_tolerance = self.audio_tolerance.clone();
            let speech_threshold = self.speech_threshold.clone();
            tokio::spawn(async move {
                while let Some(vad_audio_frame) = receiver.recv().await {
                    let audio_tolerance = if let Ok(audio_tolerance) = audio_tolerance.read() {
                        *audio_tolerance
                    } else {
                        log_warn!(
                            "Failed to lock tolerance.  Using default tolerance value: {}",
                            DEFAULT_TOLERANCE
                        );
                        DEFAULT_TOLERANCE
                    };
                    let speech_threshold = if let Ok(threshold) = speech_threshold.read() {
                        *threshold
                    } else {
                        log_warn!(
                            "Failed to lock speech threshold.  Using default threshold value: {}",
                            DEFAULT_SPEECH_THRESHOLD
                        );
                        DEFAULT_SPEECH_THRESHOLD
                    };
                    let probability = vad_audio_frame.get_probability();
                    let samples = vad_audio_frame.get_samples();
                    if probability >= speech_threshold {
                        for sample in samples {
                            speech_frame.push_front(sample);
                        }
                    }
                    probabilities.push_front(probability);
                    if probabilities
                        .iter()
                        .take(audio_tolerance)
                        .all(|&probability| probability < speech_threshold)
                    {
                        if let Err(err) = sender.send(speech_frame.make_contiguous().to_vec()).await
                        {
                            log_error!("Assembler node failed to send audio frame to receiver: {}", err);
                        }
                        speech_frame.clear();
                        probabilities.clear();
                    }
                }
            });
        }
    }
}
