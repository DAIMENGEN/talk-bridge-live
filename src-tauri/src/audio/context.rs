use crate::audio::node::concatenation::ConcatenationNode;
use crate::audio::node::gain_control::GainControlNode;
use crate::audio::node::persistence::PersistenceNode;
use crate::audio::node::speech_recognition::SpeechRecognitionNode;
use crate::audio::node::stream_input::StreamInputNode;
use crate::audio::node::text_translation::TextTranslationNode;
use crate::audio::node::vocal_isolation::VocalIsolationNode;
use crate::audio::node::voice_activity_detection::VoiceActivityDetectionNode;
use crate::audio::node::AudioNodeType;
use crate::device::input::microphone::Microphone;
use std::error::Error;
use tokio::sync::mpsc::Receiver;

pub struct AudioContext {
    microphone: Microphone,
    audio_nodes: Vec<AudioNodeType>,
}

impl AudioContext {
    pub fn new(microphone: Microphone) -> Self {
        AudioContext {
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

    pub fn connect_audio_node(&mut self, audio_node: AudioNodeType) {
        self.audio_nodes.push(audio_node);
    }

    pub fn connect_stream_input_node(&mut self, audio_node: StreamInputNode) {
        self.connect_audio_node(AudioNodeType::StreamInputNode(Box::new(audio_node)))
    }

    pub fn connect_gain_control_node(&mut self, audio_node: GainControlNode) {
        self.connect_audio_node(AudioNodeType::GainControlNode(Box::new(audio_node)))
    }

    pub fn connect_vad_node(&mut self, audio_node: VoiceActivityDetectionNode) {
        self.connect_audio_node(AudioNodeType::VoiceActivityDetectionNode(Box::new(
            audio_node,
        )))
    }

    pub fn connect_vocal_isolation_node(&mut self, audio_node: VocalIsolationNode) {
        self.connect_audio_node(AudioNodeType::VocalIsolationNode(Box::new(audio_node)))
    }

    pub fn connect_concatenation_node(&mut self, audio_node: ConcatenationNode) {
        self.connect_audio_node(AudioNodeType::ConcatenationNode(Box::new(audio_node)))
    }

    pub fn connect_speech_recognition_node(&mut self, audio_node: SpeechRecognitionNode) {
        self.connect_audio_node(AudioNodeType::SpeechRecognitionNode(Box::new(audio_node)))
    }

    pub fn connect_text_translation_node(&mut self, audio_node: TextTranslationNode) {
        self.connect_audio_node(AudioNodeType::TextTranslationNode(Box::new(audio_node)))
    }

    pub fn connect_persistence_node(&mut self, audio_node: PersistenceNode) {
        self.connect_audio_node(AudioNodeType::PersistenceNode(Box::new(audio_node)))
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
