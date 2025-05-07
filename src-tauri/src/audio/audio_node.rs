use crate::audio::audio_node::vocal_isolation_node::VocalIsolationResult;
use crate::audio::audio_node::vad_node::VADResult;
use crate::audio::AudioBlock;
use tokio::sync::mpsc::Receiver;
use crate::audio::audio_node::concatenation_node::ConcatenationResult;
use crate::audio::audio_node::speech_translator_node::SpeechTranslatorResult;

pub mod gain_node;
pub mod source_node;
pub mod vad_node;
pub mod vocal_isolation_node;
pub mod concatenation_node;
pub mod speech_translator_node;
pub mod persistence_node;

pub trait AudioNode<Input: Send, Output: Send>: Send {
    fn connect_input_source(&mut self, input_source: Receiver<Input>) -> Receiver<Output>;
    fn process(&mut self);
}

pub enum AudioNodeEnum {
    SourceNode(Box<dyn AudioNode<AudioBlock, AudioBlock>>),
    GainNode(Box<dyn AudioNode<AudioBlock, AudioBlock>>),
    VadNode(Box<dyn AudioNode<AudioBlock, VADResult>>),
    SpeechExtractorNode(Box<dyn AudioNode<VADResult, VocalIsolationResult>>),
    SpeechAssemblerNode(Box<dyn AudioNode<VocalIsolationResult, ConcatenationResult>>),
    SpeechTranslatorNode(Box<dyn AudioNode<ConcatenationResult, SpeechTranslatorResult>>),
    WavWriterNode(Box<dyn AudioNode<ConcatenationResult, ConcatenationResult>>),
}

impl AudioNodeEnum {
    pub(crate) fn process(&mut self) {
        match self {
            AudioNodeEnum::SourceNode(node) => node.process(),
            AudioNodeEnum::GainNode(node) => node.process(),
            AudioNodeEnum::VadNode(node) => node.process(),
            AudioNodeEnum::SpeechExtractorNode(node) => node.process(),
            AudioNodeEnum::SpeechAssemblerNode(node) => node.process(),
            AudioNodeEnum::SpeechTranslatorNode(node) => node.process(),
            AudioNodeEnum::WavWriterNode(node) => node.process(),
        }
    }
}
