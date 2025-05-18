use crate::audio::context::node::concatenation::ConcatenationResult;
use crate::audio::context::node::speech_recognition::SpeechRecognitionResult;
use crate::audio::context::node::text_translation::TextTranslationResult;
use crate::audio::context::node::voice_activity_detection::VoiceActivityDetectionResult;
use crate::audio::context::node::vocal_isolation::VocalIsolationResult;
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

pub trait Node<Input: Send, Output: Send>: Send {
    fn connect_input_source(&mut self, input_source: Receiver<Input>) -> Receiver<Output>;
    fn activate(&mut self);
    fn deactivate(&mut self);
}

pub enum NodeType {
    StreamInputNode(Box<dyn Node<AudioBlock, AudioBlock>>), // 麦克风或音频接口采集原始音频信号节点
    GainControlNode(Box<dyn Node<AudioBlock, AudioBlock>>), // 音频文件读取音频数据节点
    PitchDetectorNode(Box<dyn Node<AudioBlock, AudioBlock>>), // 音高检测节点 (情感识别, 音高波动可反映情绪（如愤怒 vs 平静）)
    EnergyDetectorNode(Box<dyn Node<AudioBlock, AudioBlock>>), // 音频能量检测节点
    NoiseReductionNode(Box<dyn Node<AudioBlock, AudioBlock>>), // 音频数据降噪节点
    VoiceActivityDetectionNode(Box<dyn Node<AudioBlock, VoiceActivityDetectionResult>>), // 语音活动检测节点
    VocalIsolationNode(Box<dyn Node<VoiceActivityDetectionResult, VocalIsolationResult>>), // 人声提取分离节点
    ConcatenationNode(Box<dyn Node<VocalIsolationResult, ConcatenationResult>>), // 音频拼接节点
    SpeechRecognitionNode(Box<dyn Node<ConcatenationResult, SpeechRecognitionResult>>), // 语音识别节点
    TextTranslationNode(Box<dyn Node<SpeechRecognitionResult, TextTranslationResult>>), // 文本翻译节点
    PersistenceNode(Box<dyn Node<TextTranslationResult, TextTranslationResult>>), // 音频持久化节点
}

impl NodeType {
    pub(crate) fn activate(&mut self) {
        match self {
            NodeType::StreamInputNode(node) => node.activate(),
            NodeType::GainControlNode(node) => node.activate(),
            NodeType::PitchDetectorNode(node) => node.activate(),
            NodeType::EnergyDetectorNode(node) => node.activate(),
            NodeType::NoiseReductionNode(node) => node.activate(),
            NodeType::VoiceActivityDetectionNode(node) => node.activate(),
            NodeType::VocalIsolationNode(node) => node.activate(),
            NodeType::ConcatenationNode(node) => node.activate(),
            NodeType::SpeechRecognitionNode(node) => node.activate(),
            NodeType::TextTranslationNode(node) => node.activate(),
            NodeType::PersistenceNode(node) => node.activate(),
        }
    }
    
    pub(crate) fn deactivate(&mut self) {
        match self {
            NodeType::StreamInputNode(node) => node.deactivate(),
            NodeType::GainControlNode(node) => node.deactivate(),
            NodeType::PitchDetectorNode(node) => node.deactivate(),
            NodeType::EnergyDetectorNode(node) => node.deactivate(),
            NodeType::NoiseReductionNode(node) => node.deactivate(),
            NodeType::VoiceActivityDetectionNode(node) => node.deactivate(),
            NodeType::VocalIsolationNode(node) => node.deactivate(),
            NodeType::ConcatenationNode(node) => node.deactivate(),
            NodeType::SpeechRecognitionNode(node) => node.deactivate(),
            NodeType::TextTranslationNode(node) => node.deactivate(),
            NodeType::PersistenceNode(node) => node.deactivate(),
        }
    }
}
