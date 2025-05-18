pub mod transcription;
pub mod context;
pub type AudioSample = f32;
pub type AudioBlock = Vec<AudioSample>;


// 音频帧：每一帧包含了一组样本，通常与音频的通道数量（例如立体声是2个通道）相关联。例如，在立体声的情况下，每一帧包含两个样本：一个代表左声道，另一个代表右声道。如果是单声道，则每一帧只有一个样本。
// 音频样本：表示音频信号的单一数据点，通常是一个浮动的值，代表某一时刻的音频信号强度。每个样本通常对应于特定的时间点。
// 音频块：在某些情况下，多个音频帧会被聚集成一个更大的数据块，这个数据块可以在处理时作为一个整体进行操作。通常这些块的大小是根据一定的时间长度或者其他特定需求来确定的。

// Audio Frame: An audio frame contains a group of samples, typically associated with the number of audio channels (for example, 2 channels for stereo).
// In the case of stereo, each frame contains two samples: one for the left channel and one for the right channel. If the audio is mono, each frame contains only a single sample.

// Audio Sample: An audio sample represents a single data point of the audio signal, usually a floating-point value that reflects the amplitude of the sound at a specific point in time.
// Each sample corresponds to a precise moment in the audio stream.

// Audio Block: In some contexts, multiple audio frames are grouped together into a larger data block, which can be processed as a whole.
// The size of these blocks is usually determined based on a specific time duration or other processing requirements.