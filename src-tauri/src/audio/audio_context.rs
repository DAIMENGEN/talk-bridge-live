use crate::audio::node::gain_node::GainNode;
use crate::audio::node::source_node::SourceNode;
use crate::audio::node::vad_node::VadNode;
use crate::audio::node::AudioNode;
use crate::device::input::microphone::Microphone;
use std::error::Error;
use tokio::sync::mpsc::Receiver;

pub struct AudioContext {
    microphone: Microphone,
    vad_node: Option<VadNode>,
    gain_node: Option<GainNode>,
    source_node: Option<SourceNode>,
}

impl AudioContext {
    pub fn new(microphone: Microphone) -> Self {
        AudioContext {
            microphone,
            vad_node: None,
            gain_node: None,
            source_node: None,
        }
    }

    pub fn init(&mut self) -> Result<Receiver<Vec<f32>>, Box<dyn Error>> {
        let receiver = self.microphone.init()?;
        Ok(receiver)
    }

    pub fn start(&mut self) {
        self.microphone.play();
        if let Some(source_node) = self.source_node.as_mut() {
            source_node.process();
        }
        if let Some(gain_node) = self.gain_node.as_mut() {
            gain_node.process();
        }
        if let Some(vad_node) = self.vad_node.as_mut() {
            vad_node.process();
        }
    }

    pub fn close(&mut self) {
        self.microphone.pause();
    }

    pub fn connect_source_node(&mut self, audio_node: SourceNode) {
        self.source_node = Some(audio_node);
    }

    pub fn connect_vad_node(&mut self, audio_node: VadNode) {
        self.vad_node = Some(audio_node);
    }

    pub fn connect_gain_node(&mut self, audio_node: GainNode) {
        self.gain_node = Some(audio_node);
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
}
