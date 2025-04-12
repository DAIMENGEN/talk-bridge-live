pub struct GainNode {
    gain: f32,
}

impl GainNode {
    pub fn new(gain: f32) -> Self {
        GainNode { gain }
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    pub fn get_gain(&self) -> f32 {
        self.gain
    }

    pub fn process(&self, input: &[f32]) -> Vec<f32> {
        input.iter().map(|&x| x * self.gain).collect()
    }
}