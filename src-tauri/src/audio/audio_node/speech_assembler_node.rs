use crate::app_state::DEFAULT_SPEECH_MERGE_THRESHOLD;
use crate::audio::audio_node::speech_extractor_node::SpeechExtractorResult;
use crate::audio::audio_node::AudioNode;
use crate::audio::AudioFrame;
use crate::log_error;
use chrono::{DateTime, Local};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct SpeechAssemblerNode {
    speech_merge_threshold: Arc<RwLock<f32>>,
    sender: Sender<AudioFrame>,
    input_source: Option<Receiver<SpeechExtractorResult>>,
    output_source: Option<Receiver<AudioFrame>>,
}

impl SpeechAssemblerNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = channel::<AudioFrame>(channel_capacity);
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

impl AudioNode<SpeechExtractorResult, AudioFrame> for SpeechAssemblerNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<SpeechExtractorResult>,
    ) -> Receiver<AudioFrame> {
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
                let mut audio_frame = VecDeque::<f32>::new();
                let mut prev_end_record_time_option: Option<DateTime<Local>> = None;
                while let Some(result) = receiver.recv().await {
                    let speech_merge_threshold = speech_merge_threshold
                        .read()
                        .map_or(DEFAULT_SPEECH_MERGE_THRESHOLD, |threshold| *threshold);
                    let samples = result.samples();
                    let end_record_time = result.end_record_time();
                    let start_record_time = result.start_record_time();
                    match prev_end_record_time_option {
                        Some(prev_end_record_time) => {
                            let duration =
                                start_record_time.signed_duration_since(prev_end_record_time);
                            let duration_millis = duration.num_milliseconds();
                            if duration_millis > (speech_merge_threshold * 1000f32) as i64 {
                                audio_frame.clear();
                                prev_end_record_time_option.replace(end_record_time.clone());
                            }
                            for sample in samples {
                                audio_frame.push_front(sample.clone());
                            }
                        }
                        None => {
                            for sample in samples {
                                audio_frame.push_front(sample.clone());
                            }
                            prev_end_record_time_option.replace(end_record_time.clone());
                        }
                    }
                    if let Err(err) = sender.send(audio_frame.make_contiguous().to_vec()).await {
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
