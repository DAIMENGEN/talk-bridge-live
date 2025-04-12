use voice_activity_detector::VoiceActivityDetector;

pub type Vad = VoiceActivityDetector;

pub struct VadNode {
    vad: Vad,
    chunk_size: usize,
    sample_rate: u32,
    speech_threshold: f32,
}

impl VadNode {
    pub fn new(sample_rate: u32, chunk_size: usize, speech_threshold: f32) -> Self {
        let vad = match VoiceActivityDetector::builder()
            .sample_rate(sample_rate)
            .chunk_size(chunk_size)
            .build()
        {
            Ok(a) => a,
            Err(e) => panic!("VoiceActivityDetector build failed: {}", e),
        };
        VadNode {
            vad,
            chunk_size,
            sample_rate,
            speech_threshold,
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
