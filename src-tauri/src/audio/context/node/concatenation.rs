use crate::audio::context::node::vocal_isolation::VocalIsolationResult;
use crate::audio::context::node::Node;
use crate::audio::{AudioBlock, AudioSample};
use crate::{log_info, log_warn};
use chrono::{DateTime, Local};
use std::collections::VecDeque;
use std::sync::RwLock;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct ConcatenationResult {
    start_record_time: DateTime<Local>,
    end_record_time: DateTime<Local>,
    speech_data: AudioBlock,
}

impl ConcatenationResult {
    pub fn new(
        start_record_time: DateTime<Local>,
        end_record_time: DateTime<Local>,
        speech_data: AudioBlock,
    ) -> Self {
        ConcatenationResult {
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

    #[allow(dead_code)]
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

pub struct ConcatenationNode {
    sender: Option<Sender<ConcatenationResult>>,
    input_source: Option<Receiver<VocalIsolationResult>>,
    output_source: Option<Receiver<ConcatenationResult>>,
}

impl ConcatenationNode {
    pub fn new(channel_capacity: usize) -> Self {
        let (sender, output_source) = channel::<ConcatenationResult>(channel_capacity);
        ConcatenationNode {
            sender: Some(sender),
            input_source: None,
            output_source: Some(output_source),
        }
    }
}

impl Node<VocalIsolationResult, ConcatenationResult> for ConcatenationNode {
    fn connect_input_source(
        &mut self,
        input_source: Receiver<VocalIsolationResult>,
    ) -> Receiver<ConcatenationResult> {
        self.input_source = Some(input_source);
        self.output_source.take().unwrap_or_else(|| {
            panic!("Failed to take output source from concatenation node: output source is None")
        })
    }

    fn activate(&mut self) {
        if let Some(mut receiver) = self.input_source.take() {
            let sender = self.sender.clone();
            let audio_gap_threshold = 1f32;
            tokio::spawn(async move {
                let mut audio_block = VecDeque::<AudioSample>::new();
                let mut start_record_time_option: Option<DateTime<Local>> = None;
                let mut end_record_time_option: Option<DateTime<Local>> = None;
                while let Some(result) = receiver.recv().await {
                    let speech_data = result.speech_data();
                    let current_end_record_time = result.end_record_time();
                    let current_start_record_time = result.start_record_time();
                    if start_record_time_option.is_none() {
                        start_record_time_option.replace(current_start_record_time.clone());
                    }
                    if let Some(prev_end_record_time) = end_record_time_option {
                        let duration =
                            current_start_record_time.signed_duration_since(prev_end_record_time);
                        let duration_millis = duration.num_milliseconds();
                        if duration_millis > (audio_gap_threshold * 1000f32) as i64 {
                            audio_block.clear();
                            start_record_time_option.replace(current_start_record_time.clone());
                        }
                    }
                    audio_block.extend(speech_data);
                    end_record_time_option.replace(current_end_record_time.clone());
                    let speech_assembler_result = ConcatenationResult::new(
                        start_record_time_option.unwrap(),
                        end_record_time_option.unwrap(),
                        audio_block.make_contiguous().to_vec(),
                    );
                    if let Err(err) = sender.as_ref().unwrap().send(speech_assembler_result).await {
                        log_warn!(
                            "Concatenation node failed to send audio data to the output source: {}",
                            err
                        );
                    }
                }
                log_info!("Concatenation node has been stopped.");
            });
        }
    }

    fn deactivate(&mut self) {
        self.sender = None;
    }
}
