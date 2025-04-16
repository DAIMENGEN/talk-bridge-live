use crate::log_error;
use crate::silero_vad::VadSampleRate;
use hound::{SampleFormat, WavSpec, WavWriter};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Cursor};
use std::path::{Path, PathBuf};

const SPECIFIED_CHANNELS: u16 = 1;

const SPECIFIED_SAMPLE_RATE: u32 = VadSampleRate::_16Khz as u32;

const SPECIFIED_BITS_PER_SAMPLE: u16 = 16;

fn f32_to_i16(sample: f32) -> i16 {
    let clamped = sample.clamp(-1.0, 1.0);
    (clamped * i16::MAX as f32) as i16
}

fn create_wav_spec() -> WavSpec {
    WavSpec {
        channels: SPECIFIED_CHANNELS,
        sample_rate: SPECIFIED_SAMPLE_RATE,
        bits_per_sample: SPECIFIED_BITS_PER_SAMPLE,
        sample_format: SampleFormat::Int,
    }
}

pub fn create_writer(file_path: &str) -> WavWriter<BufWriter<File>> {
    let spec = create_wav_spec();
    let file_path = Path::new(file_path);
    if let Some(parent) = file_path.parent() {
        if let Err(err) = fs::create_dir_all(parent) {
            log_error!("Failed to create directory {:?}: {}", parent, err);
            panic!("Failed to create directory {:?}: {}", parent, err);
        }
    }
    match WavWriter::create(file_path, spec) {
        Ok(writer) => writer,
        Err(err) => {
            log_error!("Failed to create WavWriter: {}", err);
            panic!("Failed to create WavWriter: {}", err);
        }
    }
}

pub fn save_wav_file_async(file_path: PathBuf, samples: &[f32]) {
    let samples = samples.to_vec();
    let file_path = file_path.to_string_lossy().to_string();
    tokio::spawn(async move {
        let mut writer = create_writer(file_path.as_str());
        samples.into_iter().for_each(|sample| {
            let i16_sample = f32_to_i16(sample);
            if let Err(err) = writer.write_sample(i16_sample) {
                log_error!("Failed to write sample to WavWriter: {}", err);
                panic!("Failed to write sample to WavWriter: {}", err);
            }
        });
        if let Err(err) = writer.flush() {
            log_error!("Failed to flush WavWriter: {}", err);
            panic!("Failed to flush WavWriter: {}", err);
        }
        if let Err(err) = writer.finalize() {
            log_error!("Failed to finalize WavWriter: {}", err);
            panic!("Failed to finalize WavWriter: {}", err);
        }
    });
}

pub fn convert_f32_to_wav_data(samples: &[f32]) -> Result<Vec<u8>, Box<dyn Error>> {
    let wav_data: Vec<u8> = Vec::new();
    let wav_spec = create_wav_spec();
    let mut cursor = Cursor::new(wav_data);
    {
        let mut writer = WavWriter::new(&mut cursor, wav_spec)?;
        for &sample in samples {
            let int_sample = f32_to_i16(sample);
            writer.write_sample(int_sample)?;
        }
        writer.flush()?;
    }
    Ok(cursor.into_inner())
}
