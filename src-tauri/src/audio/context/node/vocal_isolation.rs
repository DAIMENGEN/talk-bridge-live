use crate::app_state::{DEFAULT_SILENCE_STREAK_THRESHOLD, DEFAULT_SPEECH_THRESHOLD};
use crate::audio::context::node::voice_activity_detection::VoiceActivityDetectionResult;
use crate::audio::context::node::Node;
use crate::audio::{AudioBlock, AudioSample};
use crate::{log_info, log_warn};
use chrono::{DateTime, Local};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct VocalIsolationResult {
    start_record_time: DateTime<Local>,
    end_record_time: DateTime<Local>,
    speech_data: AudioBlock,
}

impl VocalIsolationResult {
    pub fn new(
        start_record_time: DateTime<Local>,
        end_record_time: DateTime<Local>,
        speech_data: AudioBlock,
    ) -> Self {
        VocalIsolationResult {
            start_record_time,
            end_record_time,
            speech_data,
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

    pub fn speech_data(&self) -> &AudioBlock {
        &self.speech_data
    }

    #[allow(dead_code)]
    pub fn into_speech_data(self) -> AudioBlock {
        self.speech_data
    }
}

pub struct VocalIsolationNode {
    speech_threshold: Arc<RwLock<f32>>,
    silence_streak_threshold: Arc<RwLock<usize>>,
    sender: Option<Sender<VocalIsolationResult>>,
    input_source: Option<Receiver<VoiceActivityDetectionResult>>,
    output_source: Option<Receiver<VocalIsolationResult>>,
}

impl VocalIsolationNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = mpsc::channel::<VocalIsolationResult>(channel_capacity);
        VocalIsolationNode {
            speech_threshold: Arc::new(RwLock::new(DEFAULT_SPEECH_THRESHOLD)),
            silence_streak_threshold: Arc::new(RwLock::new(DEFAULT_SILENCE_STREAK_THRESHOLD)),
            sender: Some(sender),
            input_source: None,
            output_source: Some(output_source),
        }
    }

    pub fn set_speech_threshold(&mut self, speech_threshold: Arc<RwLock<f32>>) {
        self.speech_threshold = speech_threshold;
    }

    pub fn set_silence_streak_threshold(&mut self, silence_streak_threshold: Arc<RwLock<usize>>) {
        self.silence_streak_threshold = silence_streak_threshold;
    }
}

impl Node<VoiceActivityDetectionResult, VocalIsolationResult> for VocalIsolationNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<VoiceActivityDetectionResult>,
    ) -> Receiver<VocalIsolationResult> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            panic!("Failed to take output source from vocal isolation node: output source is None")
        })
    }

    fn activate(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let sender = self.sender.clone();
            let mut probabilities = VecDeque::<f32>::new();
            let mut speech_audio_block = VecDeque::<AudioSample>::new();
            let speech_threshold = self.speech_threshold.clone();
            let silence_streak_threshold = self.silence_streak_threshold.clone();
            tokio::spawn(async move {
                let mut start_record_time: Option<DateTime<Local>> = None;
                while let Some(result) = receiver.recv().await {
                    let silence_streak_threshold = silence_streak_threshold
                        .read()
                        .map_or(DEFAULT_SILENCE_STREAK_THRESHOLD, |silence_streak_count| {
                            *silence_streak_count
                        });
                    let speech_threshold = speech_threshold
                        .read()
                        .map_or(DEFAULT_SPEECH_THRESHOLD, |threshold| *threshold);
                    let probability = result.probability();
                    probabilities.push_front(probability);
                    if probability >= speech_threshold {
                        if start_record_time.is_none() {
                            start_record_time.replace(Local::now());
                        }
                        speech_audio_block.extend(result.samples());
                    }
                    if start_record_time.is_some()
                        && probabilities
                            .iter()
                            .take(silence_streak_threshold)
                            .all(|&probability| probability < speech_threshold)
                    {
                        let end_time = Local::now();
                        let start_time = start_record_time.take().unwrap();
                        if (end_time - start_time).num_milliseconds() >= 500 {
                            let speech_extractor_result = VocalIsolationResult::new(
                                start_time,
                                end_time,
                                speech_audio_block.make_contiguous().to_vec(),
                            );
                            if let Err(err) =
                                sender.as_ref().unwrap().send(speech_extractor_result).await
                            {
                                log_warn!("Vocal isolation node failed to send audio data to the output source: {}", err);
                            }
                            probabilities.clear();
                            speech_audio_block.clear();
                        } else {
                            start_record_time.replace(start_time);
                        }
                    }
                }
                log_info!("Vocal isolation node has been stopped.");
            });
        }
    }

    fn deactivate(&mut self) {
        self.sender = None;
    }
}
