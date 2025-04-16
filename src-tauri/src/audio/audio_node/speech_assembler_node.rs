use crate::app_state::DEFAULT_SPEECH_MERGE_THRESHOLD;
use crate::audio::audio_node::speech_extractor_node::SpeechExtractorResult;
use crate::audio::audio_node::AudioNode;
use crate::audio::{AudioBlock, AudioSample};
use crate::log_error;
use chrono::{DateTime, Local};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct SpeechAssemblerResult {
    start_record_time: DateTime<Local>,
    end_record_time: DateTime<Local>,
    samples: AudioBlock,
}

impl SpeechAssemblerResult {
    pub fn new(
        start_record_time: DateTime<Local>,
        end_record_time: DateTime<Local>,
        samples: AudioBlock,
    ) -> Self {
        SpeechAssemblerResult {
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

    pub fn samples(&self) -> &AudioBlock {
        &self.samples
    }

    #[allow(dead_code)]
    pub fn into_samples(self) -> AudioBlock {
        self.samples
    }
}

pub struct SpeechAssemblerNode {
    speech_merge_threshold: Arc<RwLock<f32>>,
    sender: Sender<SpeechAssemblerResult>,
    input_source: Option<Receiver<SpeechExtractorResult>>,
    output_source: Option<Receiver<SpeechAssemblerResult>>,
}

impl SpeechAssemblerNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = channel::<SpeechAssemblerResult>(channel_capacity);
        SpeechAssemblerNode {
            speech_merge_threshold: Arc::new(RwLock::new(DEFAULT_SPEECH_MERGE_THRESHOLD)),
            sender,
            input_source: None,
            output_source: Some(output_source),
        }
    }

    pub fn set_merge_threshold(&mut self, merge_threshold: Arc<RwLock<f32>>) {
        self.speech_merge_threshold = merge_threshold;
    }
}

impl AudioNode<SpeechExtractorResult, SpeechAssemblerResult> for SpeechAssemblerNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<SpeechExtractorResult>,
    ) -> Receiver<SpeechAssemblerResult> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            log_error!("Speech context node output source is None");
            panic!("Speech context node output source is None")
        })
    }

    fn process(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let sender = self.sender.clone();
            let speech_merge_threshold = self.speech_merge_threshold.clone();
            tokio::spawn(async move {
                let mut audio_block = VecDeque::<AudioSample>::new();
                let mut start_record_time_option: Option<DateTime<Local>> = None;
                let mut end_record_time_option: Option<DateTime<Local>> = None;
                while let Some(result) = receiver.recv().await {
                    let samples = result.samples();
                    let current_end_record_time = result.end_record_time();
                    let current_start_record_time = result.start_record_time();
                    let speech_merge_threshold = speech_merge_threshold
                        .read()
                        .map_or(DEFAULT_SPEECH_MERGE_THRESHOLD, |threshold| *threshold);
                    if start_record_time_option.is_none() {
                        start_record_time_option.replace(current_start_record_time.clone());
                    }
                    match end_record_time_option {
                        Some(prev_end_record_time) => {
                            let duration = current_start_record_time.signed_duration_since(prev_end_record_time);
                            let duration_millis = duration.num_milliseconds();
                            if duration_millis > (speech_merge_threshold * 1000f32) as i64 {
                                audio_block.clear();
                                start_record_time_option.replace(current_start_record_time.clone());
                            }
                            audio_block.extend(samples);
                            end_record_time_option.replace(current_end_record_time.clone());
                        }
                        None => {
                            audio_block.extend(samples);
                            end_record_time_option.replace(current_end_record_time.clone());
                        }
                    }
                    let speech_assembler_result = SpeechAssemblerResult::new(
                       start_record_time_option.unwrap(),
                       end_record_time_option.unwrap(),
                       audio_block.make_contiguous().to_vec(),
                    );
                    if let Err(err) = sender.send(speech_assembler_result).await {
                        log_error!(
                            "Speech context node failed to send audio frame to receiver: {}",
                            err
                        );
                    }
                }
            });
        }
    }
}
