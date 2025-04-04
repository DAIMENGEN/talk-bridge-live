use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{BufferSize, Device, SampleFormat, StreamConfig};
use log::{error, info};
use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

pub struct Microphone {
    pub device: Device,
    stream: Option<cpal::Stream>,
}

impl Microphone {
    pub fn new(device: Device) -> Self {
        Microphone {
            device,
            stream: None,
        }
    }
    pub fn play(&mut self) {
        if let Some(stream) = &self.stream {
            match stream.play() {
                Ok(_) => {
                    info!(
                        "Microphone {} stream was started.",
                        self.get_microphone_name()
                    );
                }
                Err(err) => {
                    error!(
                        "Microphone {} stream start error: {}",
                        self.get_microphone_name(),
                        err
                    );
                }
            }
        }
    }
    pub fn pause(&mut self) {
        if let Some(stream) = &self.stream {
            match stream.pause() {
                Ok(_) => {
                    info!(
                        "Microphone {} stream was paused.",
                        self.get_microphone_name()
                    );
                }
                Err(err) => {
                    error!(
                        "Microphone {} stream pause error: {}",
                        self.get_microphone_name(),
                        err
                    );
                }
            }
        }
    }
    pub fn initialization(&mut self) -> Result<Receiver<Vec<f32>>, Box<dyn Error>> {
        let config = self.get_microphone_config();
        let microphone_name = self.get_microphone_name();
        let sample_format = self.get_microphone_sample_format();
        let (tx, rx) = mpsc::channel::<Vec<f32>>(100);
        match sample_format {
            SampleFormat::F32 => {
                let stream = self.device.build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        let mut buffer = vec![0.0; data.len()];
                        buffer.copy_from_slice(data);
                        tx.blocking_send(buffer).unwrap();
                    },
                    move |err| {
                        error!(
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
                error!(
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
    pub fn get_microphone_name(&self) -> String {
        self.device.name().unwrap_or_else(|_| {
            error!("Failed to get Microphone name.");
            panic!("Failed to get Microphone name.");
        })
    }
    pub fn get_microphone_config(&self) -> StreamConfig {
        match self.device.default_input_config() {
            Ok(config) => StreamConfig {
                channels: config.channels(),
                sample_rate: config.sample_rate(),
                buffer_size: BufferSize::Fixed(512u32),
            },
            Err(_) => {
                error!("Failed to get Microphone config.");
                panic!("Failed to get Microphone config.");
            }
        }
    }
    pub fn get_microphone_channels(&self) -> u16 {
        self.device
            .default_input_config()
            .unwrap_or_else(|_| {
                error!("Failed to get Microphone channels.");
                panic!("Failed to get Microphone channels.");
            })
            .channels()
    }
    pub fn get_microphone_sample_rate(&self) -> u32 {
        self.device
            .default_input_config()
            .unwrap_or_else(|_| {
                error!("Failed to get Microphone sample rate.");
                panic!("Failed to get Microphone sample rate.");
            })
            .sample_rate()
            .0
    }
    pub fn get_microphone_sample_format(&self) -> SampleFormat {
        self.device
            .default_input_config()
            .unwrap_or_else(|_| {
                error!("Failed to get Microphone sample format.");
                panic!("Failed to get Microphone sample format.");
            })
            .sample_format()
    }
}

// https://github.com/RustAudio/cpal/issues/818#event-16783007976
unsafe impl Send for Microphone {}

// https://github.com/RustAudio/cpal/issues/818#event-16783007976
unsafe impl Sync for Microphone {}
