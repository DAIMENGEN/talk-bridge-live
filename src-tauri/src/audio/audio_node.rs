use crate::audio::audio_node::speech_extractor_node::SpeechAudioFrame;
use crate::audio::audio_node::vad_node::VadAudioFrame;
use crate::audio::AudioFrame;
use tokio::sync::mpsc::Receiver;

pub mod gain_node;
pub mod source_node;
pub mod vad_node;
pub mod speech_extractor_node;
pub mod speech_context_node;

pub trait AudioNode<Input: Send, Output: Send>: Send {
    fn connect_input_source(&mut self, input_source: Receiver<Input>) -> Receiver<Output>;
    fn process(&mut self);
}

pub enum AudioNodeEnum {
    SourceNode(Box<dyn AudioNode<AudioFrame, AudioFrame>>),
    GainNode(Box<dyn AudioNode<AudioFrame, AudioFrame>>),
    VadNode(Box<dyn AudioNode<AudioFrame, VadAudioFrame>>),
    SpeechExtractorNode(Box<dyn AudioNode<VadAudioFrame, SpeechAudioFrame>>),
    SpeechContextNode(Box<dyn AudioNode<SpeechAudioFrame, AudioFrame>>)
}

impl AudioNodeEnum {
    pub(crate) fn process(&mut self) {
        match self {
            AudioNodeEnum::SourceNode(node) => node.process(),
            AudioNodeEnum::GainNode(node) => node.process(),
            AudioNodeEnum::VadNode(node) => node.process(),
            AudioNodeEnum::SpeechExtractorNode(node) => node.process(),
            AudioNodeEnum::SpeechContextNode(node) => node.process(),
        }
    }
}
