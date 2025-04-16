use crate::app_state::{DEFAULT_SPEECH_THRESHOLD, DEFAULT_TOLERANCE};
use crate::audio::audio_node::vad_node::VADResult;
use crate::audio::audio_node::AudioNode;
use crate::audio::AudioFrame;
use crate::log_error;
use chrono::{DateTime, Local};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct SpeechExtractorResult {
    start_record_time: DateTime<Local>,
    end_record_time: DateTime<Local>,
    samples: AudioFrame,
}

impl SpeechExtractorResult {
    pub fn new(
        start_record_time: DateTime<Local>,
        end_record_time: DateTime<Local>,
        samples: AudioFrame,
    ) -> Self {
        SpeechExtractorResult {
            start_record_time,
            end_record_time,
            samples,
        }
    }

    pub fn start_record_time(&self) -> &DateTime<Local> {
        &self.start_record_time
    }

    #[allow(dead_code)]
    pub fn into_start_record_time(self) -> DateTime<Local> {
        self.start_record_time
    }

    pub fn end_record_time(&self) -> &DateTime<Local> {
        &self.end_record_time
    }

    #[allow(dead_code)]
    pub fn into_end_record_time(self) -> DateTime<Local> {
        self.end_record_time
    }

    pub fn samples(&self) -> &AudioFrame {
        &self.samples
    }

    #[allow(dead_code)]
    pub fn into_samples(self) -> AudioFrame {
        self.samples
    }
}

pub struct SpeechExtractorNode {
    audio_tolerance: Arc<RwLock<usize>>,
    speech_threshold: Arc<RwLock<f32>>,
    sender: Sender<SpeechExtractorResult>,
    input_source: Option<Receiver<VADResult>>,
    output_source: Option<Receiver<SpeechExtractorResult>>,
}

impl SpeechExtractorNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = mpsc::channel::<SpeechExtractorResult>(channel_capacity);
        SpeechExtractorNode {
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

impl AudioNode<VADResult, SpeechExtractorResult> for SpeechExtractorNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<VADResult>,
    ) -> Receiver<SpeechExtractorResult> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            log_error!("Reassembly node output source is None");
            panic!("Reassembly node output source is None")
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
                let mut start_record_time: Option<DateTime<Local>> = None;
                while let Some(result) = receiver.recv().await {
                    let audio_tolerance = audio_tolerance
                        .read()
                        .map_or(DEFAULT_TOLERANCE, |tolerance| *tolerance);
                    let speech_threshold = speech_threshold
                        .read()
                        .map_or(DEFAULT_SPEECH_THRESHOLD, |threshold| *threshold);
                    let probability = result.probability();
                    let samples = result.samples();
                    if probability >= speech_threshold {
                        if start_record_time.is_none() {
                            start_record_time.replace(Local::now());
                        }
                        speech_frame.extend(samples);
                    }
                    probabilities.push_front(probability);
                    if start_record_time.is_some()
                        && probabilities
                            .iter()
                            .take(audio_tolerance)
                            .all(|&probability| probability < speech_threshold)
                    {
                        let speech_audio_frame = SpeechExtractorResult::new(
                            start_record_time.take().unwrap(),
                            Local::now(),
                            speech_frame.make_contiguous().to_vec(),
                        );
                        if let Err(err) = sender.send(speech_audio_frame).await {
                            log_error!(
                                "Reassembly node failed to send audio frame to receiver: {}",
                                err
                            );
                        }
                        speech_frame.clear();
                        probabilities.clear();
                    }
                }
            });
        }
    }
}
