use crate::audio::AudioFrame;
use tokio::sync::mpsc::Receiver;

pub mod vad_node;
pub mod gain_node;
pub mod source_node;

pub trait AudioNode<Output: Send>: Send {
    fn connect_input_source(&mut self, input_source: Receiver<AudioFrame>) -> Receiver<Output>;
    fn process(&mut self);
}