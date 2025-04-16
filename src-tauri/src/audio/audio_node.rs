use crate::audio::audio_node::speech_extractor_node::SpeechExtractorResult;
use crate::audio::audio_node::vad_node::VADResult;
use crate::audio::AudioBlock;
use tokio::sync::mpsc::Receiver;
use crate::audio::audio_node::speech_assembler_node::SpeechAssemblerResult;
use crate::audio::audio_node::speech_translator_node::SpeechTranslatorResult;

pub mod gain_node;
pub mod source_node;
pub mod vad_node;
pub mod speech_extractor_node;
pub mod speech_assembler_node;
pub mod speech_translator_node;
pub mod wav_writer_node;

pub trait AudioNode<Input: Send, Output: Send>: Send {
    fn connect_input_source(&mut self, input_source: Receiver<Input>) -> Receiver<Output>;
    fn process(&mut self);
}

pub enum AudioNodeEnum {
    SourceNode(Box<dyn AudioNode<AudioBlock, AudioBlock>>),
    GainNode(Box<dyn AudioNode<AudioBlock, AudioBlock>>),
    VadNode(Box<dyn AudioNode<AudioBlock, VADResult>>),
    SpeechExtractorNode(Box<dyn AudioNode<VADResult, SpeechExtractorResult>>),
    SpeechAssemblerNode(Box<dyn AudioNode<SpeechExtractorResult, SpeechAssemblerResult>>),
    SpeechTranslatorNode(Box<dyn AudioNode<SpeechAssemblerResult, SpeechTranslatorResult>>),
    WavWriterNode(Box<dyn AudioNode<SpeechAssemblerResult, SpeechAssemblerResult>>),
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
