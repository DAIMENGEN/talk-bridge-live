use crate::audio::node::concatenation::ConcatenationResult;
use crate::audio::node::speech_recognition::SpeechRecognitionResult;
use crate::audio::node::text_translation::TextTranslationResult;
use crate::audio::node::voice_activity_detection::VoiceActivityDetectionResult;
use crate::audio::node::vocal_isolation::VocalIsolationResult;
use crate::audio::AudioBlock;
use tokio::sync::mpsc::Receiver;

pub mod concatenation;
pub mod gain_control;
pub mod persistence;
pub mod stream_input;
pub mod voice_activity_detection;
pub mod vocal_isolation;
pub mod speech_recognition;
pub mod text_translation;

pub trait AudioNode<Input: Send, Output: Send>: Send {
    fn connect_input_source(&mut self, input_source: Receiver<Input>) -> Receiver<Output>;
    fn activate(&mut self);
    fn deactivate(&mut self);
}

pub enum AudioNodeType {
    StreamInputNode(Box<dyn AudioNode<AudioBlock, AudioBlock>>), // 麦克风或音频接口采集原始音频信号节点
    GainControlNode(Box<dyn AudioNode<AudioBlock, AudioBlock>>), // 音频文件读取音频数据节点
    PitchDetectorNode(Box<dyn AudioNode<AudioBlock, AudioBlock>>), // 音高检测节点 (情感识别, 音高波动可反映情绪（如愤怒 vs 平静）)
    EnergyDetectorNode(Box<dyn AudioNode<AudioBlock, AudioBlock>>), // 音频能量检测节点
    NoiseReductionNode(Box<dyn AudioNode<AudioBlock, AudioBlock>>), // 音频数据降噪节点
    VoiceActivityDetectionNode(Box<dyn AudioNode<AudioBlock, VoiceActivityDetectionResult>>), // 语音活动检测节点
    VocalIsolationNode(Box<dyn AudioNode<VoiceActivityDetectionResult, VocalIsolationResult>>), // 人声提取分离节点
    ConcatenationNode(Box<dyn AudioNode<VocalIsolationResult, ConcatenationResult>>), // 音频拼接节点
    SpeechRecognitionNode(Box<dyn AudioNode<ConcatenationResult, SpeechRecognitionResult>>), // 语音识别节点
    TextTranslationNode(Box<dyn AudioNode<SpeechRecognitionResult, TextTranslationResult>>), // 文本翻译节点
    PersistenceNode(Box<dyn AudioNode<TextTranslationResult, TextTranslationResult>>), // 音频持久化节点
}

impl AudioNodeType {
    pub(crate) fn activate(&mut self) {
        match self {
            AudioNodeType::StreamInputNode(node) => node.activate(),
            AudioNodeType::GainControlNode(node) => node.activate(),
            AudioNodeType::PitchDetectorNode(node) => node.activate(),
            AudioNodeType::EnergyDetectorNode(node) => node.activate(),
            AudioNodeType::NoiseReductionNode(node) => node.activate(),
            AudioNodeType::VoiceActivityDetectionNode(node) => node.activate(),
            AudioNodeType::VocalIsolationNode(node) => node.activate(),
            AudioNodeType::ConcatenationNode(node) => node.activate(),
            AudioNodeType::SpeechRecognitionNode(node) => node.activate(),
            AudioNodeType::TextTranslationNode(node) => node.activate(),
            AudioNodeType::PersistenceNode(node) => node.activate(),
        }
    }
    
    pub(crate) fn deactivate(&mut self) {
        match self {
            AudioNodeType::StreamInputNode(node) => node.deactivate(),
            AudioNodeType::GainControlNode(node) => node.deactivate(),
            AudioNodeType::PitchDetectorNode(node) => node.deactivate(),
            AudioNodeType::EnergyDetectorNode(node) => node.deactivate(),
            AudioNodeType::NoiseReductionNode(node) => node.deactivate(),
            AudioNodeType::VoiceActivityDetectionNode(node) => node.deactivate(),
            AudioNodeType::VocalIsolationNode(node) => node.deactivate(),
            AudioNodeType::ConcatenationNode(node) => node.deactivate(),
            AudioNodeType::SpeechRecognitionNode(node) => node.deactivate(),
            AudioNodeType::TextTranslationNode(node) => node.deactivate(),
            AudioNodeType::PersistenceNode(node) => node.deactivate(),
        }
    }
}
