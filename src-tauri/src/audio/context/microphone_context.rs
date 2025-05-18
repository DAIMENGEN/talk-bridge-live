use crate::audio::context::node::concatenation::ConcatenationNode;
use crate::audio::context::node::gain_control::GainControlNode;
use crate::audio::context::node::persistence::PersistenceNode;
use crate::audio::context::node::speech_recognition::SpeechRecognitionNode;
use crate::audio::context::node::stream_input::StreamInputNode;
use crate::audio::context::node::text_translation::TextTranslationNode;
use crate::audio::context::node::vocal_isolation::VocalIsolationNode;
use crate::audio::context::node::voice_activity_detection::VoiceActivityDetectionNode;
use crate::audio::context::node::NodeType;
use crate::device::input::microphone::Microphone;
use std::error::Error;
use tokio::sync::mpsc::Receiver;

pub struct MicrophoneContext {
    microphone: Microphone,
    audio_nodes: Vec<NodeType>,
}

impl MicrophoneContext {
    pub fn new(microphone: Microphone) -> Self {
        MicrophoneContext {
            microphone,
            audio_nodes: vec![],
        }
    }

    pub fn init(&mut self) -> Result<Receiver<Vec<f32>>, Box<dyn Error>> {
        let receiver = self.microphone.init()?;
        Ok(receiver)
    }

    pub fn start(&mut self) {
        self.microphone.play();
        for audio_node in self.audio_nodes.iter_mut() {
            audio_node.activate();
        }
    }

    pub fn close(&mut self) {
        self.microphone.exit();
        for audio_node in self.audio_nodes.iter_mut() {
            audio_node.deactivate();
        }
    }

    pub fn connect_audio_node(&mut self, audio_node: NodeType) {
        self.audio_nodes.push(audio_node);
    }

    pub fn connect_stream_input_node(&mut self, audio_node: StreamInputNode) {
        self.connect_audio_node(NodeType::StreamInputNode(Box::new(audio_node)))
    }

    pub fn connect_gain_control_node(&mut self, audio_node: GainControlNode) {
        self.connect_audio_node(NodeType::GainControlNode(Box::new(audio_node)))
    }

    pub fn connect_vad_node(&mut self, audio_node: VoiceActivityDetectionNode) {
        self.connect_audio_node(NodeType::VoiceActivityDetectionNode(Box::new(
            audio_node,
        )))
    }

    pub fn connect_vocal_isolation_node(&mut self, audio_node: VocalIsolationNode) {
        self.connect_audio_node(NodeType::VocalIsolationNode(Box::new(audio_node)))
    }

    pub fn connect_concatenation_node(&mut self, audio_node: ConcatenationNode) {
        self.connect_audio_node(NodeType::ConcatenationNode(Box::new(audio_node)))
    }

    pub fn connect_speech_recognition_node(&mut self, audio_node: SpeechRecognitionNode) {
        self.connect_audio_node(NodeType::SpeechRecognitionNode(Box::new(audio_node)))
    }

    pub fn connect_text_translation_node(&mut self, audio_node: TextTranslationNode) {
        self.connect_audio_node(NodeType::TextTranslationNode(Box::new(audio_node)))
    }

    pub fn connect_persistence_node(&mut self, audio_node: PersistenceNode) {
        self.connect_audio_node(NodeType::PersistenceNode(Box::new(audio_node)))
    }

    pub fn create_stream_input_node(&self) -> StreamInputNode {
        StreamInputNode::new(1024)
    }

    pub fn create_gain_control_node(&self) -> GainControlNode {
        GainControlNode::new(1024)
    }

    pub fn create_vad_node(&mut self) -> VoiceActivityDetectionNode {
        let sample_rate = self.microphone.get_target_sample_rate() as u32;
        let chunk_size = self.microphone.get_output_frames_size() as usize;
        VoiceActivityDetectionNode::new(1024, sample_rate, chunk_size)
    }

    pub fn create_vocal_isolation_node(&self) -> VocalIsolationNode {
        VocalIsolationNode::new(1024)
    }

    pub fn create_concatenation_node(&self) -> ConcatenationNode {
        ConcatenationNode::new(1024)
    }

    pub fn create_speech_recognition_node(&self) -> SpeechRecognitionNode {
        SpeechRecognitionNode::new(1024)
    }

    pub fn create_text_translation_node(&self) -> TextTranslationNode {
        TextTranslationNode::new(1024)
    }

    pub fn create_persistence_node(&self) -> PersistenceNode {
        PersistenceNode::new(1024)
    }
}
