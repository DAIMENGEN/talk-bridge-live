#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum VadSampleRate {
    _8Khz = 8000,
    _16Khz = 16000,
}

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum VadSampleSize {
    _256 = 256, // support 8Khz
    _512 = 512, // support 8Khz and 16Khz
    _768 = 768, // support 8Khz and 16Khz
    _1024 = 1024, // support 16Khz
}