pub struct GainNode {
    gain: f32,
}

impl GainNode {
    pub fn new(gain: f32) -> Self {
        GainNode { gain }
    }

    pub fn process(&self, input: &[f32]) -> Vec<f32> {
        input.iter().map(|&x| x * self.gain).collect()
    }
}