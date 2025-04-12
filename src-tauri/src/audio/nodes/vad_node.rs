use voice_activity_detector::VoiceActivityDetector;
use crate::log_error;

// https://github.com/nkeenan38/voice_activity_detector
pub type Vad = VoiceActivityDetector;

pub struct VadNode {
    vad: Vad,
    chunk_size: usize,
}

impl VadNode {
    pub fn new(sample_rate: u32, chunk_size: usize) -> Self {
        let vad = match VoiceActivityDetector::builder()
            .sample_rate(sample_rate)
            .chunk_size(chunk_size)
            .build()
        {
            Ok(a) => a,
            Err(e) => {
                log_error!("VoiceActivityDetector build failed: {}", e);
                panic!("VoiceActivityDetector build failed: {}", e)
            },
        };
        VadNode {
            vad,
            chunk_size
        }
    }

    pub fn predict(&mut self, samples: &[f32]) -> f32 {
        let chunk_size = self.chunk_size;
        let mut chunk = vec![0f32; chunk_size];
        let len = samples.len().min(chunk_size);
        chunk[..len].copy_from_slice(&samples[..len]);
        let probability = self.vad.predict(chunk);
        probability
    }
}
