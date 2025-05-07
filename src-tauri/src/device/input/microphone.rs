use crate::silero_vad::{VadSampleRate, VadSampleSize};
use crate::{log_error, log_info};
use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{BufferSize, Device, SampleFormat, StreamConfig};
use dasp::interpolate::linear::Linear;
use dasp::{signal, Signal};
use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

pub struct Microphone {
    pub device: Device,
    stream: Option<cpal::Stream>,
    sample_rate: VadSampleRate, // Control the sample rate of the Microphone input. This is related to the sample rate required by the VAD (Voice Activity Detection).
    output_frames_size: VadSampleSize, // Control the size of the sample batch sent externally after resampling the Microphone input. This is related to the sample size required by the VAD (Voice Activity Detection).
}



impl Microphone {
    pub fn new(device: Device) -> Self {
        Microphone {
            device,
            stream: None,
            sample_rate: VadSampleRate::_16Khz,
            output_frames_size: VadSampleSize::_512,
        }
    }
    pub fn init(&mut self) -> Result<Receiver<Vec<f32>>, Box<dyn Error>> {
        let config = self.get_config();
        let channels = self.get_original_channels() as usize;
        let original_sample_rate = self.get_original_sample_rate() as usize;
        let target_sample_rate = self.sample_rate as usize;
        let output_frames_size = self.output_frames_size as usize;
        let min_frame_size = output_frames_size * (original_sample_rate / target_sample_rate);
        let microphone_name = self.get_name();
        let sample_format = self.get_original_sample_format();
        let (tx, rx) = mpsc::channel::<Vec<f32>>(100);
        let mut input_buffer: Vec<f32> = vec![];
        match sample_format {
            SampleFormat::F32 => {
                let stream = self.device.build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        let samples = Microphone::stereo_to_mono(channels, data);
                        input_buffer.extend(samples);
                        if input_buffer.len() >= min_frame_size {
                            let samples = input_buffer.drain(0..min_frame_size).collect();
                            let samples = Microphone::resample(original_sample_rate, target_sample_rate, samples);
                            if let Err(err) = tx.blocking_send(samples) {
                                log_error!("Microphone send samples error: {}", err);
                            }
                        }
                    },
                    move |err| {
                        log_error!(
                            "Microphone {} build input stream error: {}",
                            microphone_name.clone(),
                            err
                        );
                    },
                    None,
                )?;
                self.stream = Some(stream);
            }
            _ => {
                log_error!(
                    "Unsupported microphone {} sample format: {:?}",
                    microphone_name.clone(),
                    sample_format
                );
                panic!(
                    "Unsupported microphone {} sample format: {:?}",
                    microphone_name.clone(),
                    sample_format
                );
            }
        };
        Ok(rx)
    }
    #[allow(dead_code)]
    pub fn exit(&mut self) {
        if let Some(stream) = self.stream.take() {
            stream.pause().unwrap_or_else(|_| {
                self.stream = Some(stream);
                log_error!("Microphone {} stream exit error.", self.get_name());
            });
        } else { 
            log_info!("Microphone {} stream was already stopped.", self.get_name());
        }
    }
    pub fn play(&self) {
        if let Some(stream) = &self.stream {
            match stream.play() {
                Ok(_) => {
                    log_info!(
                        "Microphone {} stream was started.",
                        self.get_name()
                    );
                }
                Err(err) => {
                    log_error!(
                        "Microphone {} stream start error: {}",
                        self.get_name(),
                        err
                    );
                }
            }
        }
    }
    pub fn pause(&self) {
        if let Some(stream) = &self.stream {
            match stream.pause() {
                Ok(_) => {
                    log_info!(
                        "Microphone {} stream was paused.",
                        self.get_name()
                    );
                }
                Err(err) => {
                    log_error!(
                        "Microphone {} stream pause error: {}",
                        self.get_name(),
                        err
                    );
                }
            }
        }
    }
    pub fn get_name(&self) -> String {
        self.device.name().unwrap_or_else(|_| {
            log_error!("Failed to get Microphone name.");
            panic!("Failed to get Microphone name.");
        })
    }
    pub fn get_config(&self) -> StreamConfig {
        match self.device.default_input_config() {
            Ok(config) => StreamConfig {
                channels: config.channels(),
                sample_rate: config.sample_rate(),
                buffer_size: BufferSize::Fixed(512u32),
            },
            Err(_) => {
                log_error!("Failed to get Microphone config.");
                panic!("Failed to get Microphone config.");
            }
        }
    }
    pub fn get_target_sample_rate(&self) -> VadSampleRate {
        self.sample_rate
    }
    pub fn get_output_frames_size(&self) -> VadSampleSize {
        self.output_frames_size
    }
    pub fn get_original_channels(&self) -> u16 {
        self.device
            .default_input_config()
            .unwrap_or_else(|_| {
                log_error!("Failed to get Microphone channels.");
                panic!("Failed to get Microphone channels.");
            })
            .channels()
    }
    pub fn get_original_sample_rate(&self) -> u32 {
        self.device
            .default_input_config()
            .unwrap_or_else(|_| {
                log_error!("Failed to get Microphone sample rate.");
                panic!("Failed to get Microphone sample rate.");
            })
            .sample_rate()
            .0
    }
    pub fn get_original_sample_format(&self) -> SampleFormat {
        self.device
            .default_input_config()
            .unwrap_or_else(|_| {
                log_error!("Failed to get Microphone sample format.");
                panic!("Failed to get Microphone sample format.");
            })
            .sample_format()
    }
    pub fn stereo_to_mono(channels: usize, samples: &[f32]) -> Vec<f32> {
        if channels == 1 {
            samples.to_vec()
        } else {
            samples.chunks_exact(channels)
                .map(|frame| frame.iter().sum::<f32>() / channels as f32)
                .collect()
        }
    }
    pub fn resample(original_sample_rate: usize, target_sample_rate: usize, samples: Vec<f32>) -> Vec<f32> {
        if original_sample_rate != target_sample_rate {
            let target_size = samples.len() * target_sample_rate / original_sample_rate;
            let interpolator = Linear::new(samples[0], samples[1]);
            let source = signal::from_iter(samples.iter().cloned());
            let samples = source.from_hz_to_hz(
                interpolator,
                original_sample_rate as f64,
                target_sample_rate as f64,
            );
            samples.take(target_size).collect()
        } else {
            samples.to_vec()
        }
    }
}

// https://github.com/RustAudio/cpal/issues/818#event-16783007976
unsafe impl Send for Microphone {}

// https://github.com/RustAudio/cpal/issues/818#event-16783007976
unsafe impl Sync for Microphone {}
