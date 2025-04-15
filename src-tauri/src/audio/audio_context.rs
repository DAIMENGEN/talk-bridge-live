use crate::audio::audio_node::gain_node::GainNode;
use crate::audio::audio_node::source_node::SourceNode;
use crate::audio::audio_node::vad_node::VadNode;
use crate::audio::audio_node::AudioNodeEnum;
use crate::device::input::microphone::Microphone;
use std::error::Error;
use tokio::sync::mpsc::Receiver;
use crate::audio::audio_node::reassembly_node::ReassemblyNode;

pub struct AudioContext {
    microphone: Microphone,
    audio_nodes: Vec<AudioNodeEnum>,
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
            audio_node.process();
        }
    }

    pub fn close(&self) {
        self.microphone.pause();
    }

    pub fn connect_audio_node(&mut self, audio_node: AudioNodeEnum) {
        self.audio_nodes.push(audio_node);
    }

    pub fn connect_source_node(&mut self, audio_node: SourceNode) {
        self.connect_audio_node(AudioNodeEnum::SourceNode(Box::new(audio_node)))
    }

    pub fn connect_gain_node(&mut self, audio_node: GainNode) {
        self.connect_audio_node(AudioNodeEnum::GainNode(Box::new(audio_node)))
    }

    pub fn connect_vad_node(&mut self, audio_node: VadNode) {
        self.connect_audio_node(AudioNodeEnum::VadNode(Box::new(audio_node)))
    }

    pub fn connect_reassembly_node(&mut self, audio_node: ReassemblyNode) {
        self.connect_audio_node(AudioNodeEnum::AssemblerNode(Box::new(audio_node)))
    }

    pub fn create_source_node(&self) -> SourceNode {
        SourceNode::new(1024)
    }

    pub fn create_gain_node(&self) -> GainNode {
        GainNode::new(1024)
    }

    pub fn create_vad_node(&mut self) -> VadNode {
        let sample_rate = self.microphone.get_target_sample_rate() as u32;
        let chunk_size = self.microphone.get_output_frames_size() as usize;
        VadNode::new(1024, sample_rate, chunk_size)
    }

    pub fn create_reassembly_node(&self) -> ReassemblyNode {
        ReassemblyNode::new(1024)
    }
}
