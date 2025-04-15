use crate::audio::audio_context::AudioContext;
use std::sync::{Arc, Mutex, RwLock};
use tauri::State;

pub const DEFAULT_TOLERANCE: usize = 1;

pub const DEFAULT_MICROPHONE_GAIN: f32 = 1.0;

pub const DEFAULT_SPEECH_THRESHOLD: f32 = 0.5;

pub const DEFAULT_SPEECH_MERGE_THRESHOLD: f32 = 0.5;

pub struct AppState {
    audio_tolerance: Arc<RwLock<usize>>,
    microphone_gain: Arc<RwLock<f32>>,
    speech_threshold: Arc<RwLock<f32>>,
    speech_merge_threshold: Arc<RwLock<f32>>,
    recording_context: Arc<Mutex<Option<AudioContext>>>,
    human_voice_detection_context: Arc<Mutex<Option<AudioContext>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            audio_tolerance: Arc::new(RwLock::new(DEFAULT_TOLERANCE)),
            microphone_gain: Arc::new(RwLock::new(DEFAULT_MICROPHONE_GAIN)),
            speech_threshold: Arc::new(RwLock::new(DEFAULT_SPEECH_THRESHOLD)),
            speech_merge_threshold: Arc::new(RwLock::new(DEFAULT_SPEECH_MERGE_THRESHOLD)),
            recording_context: Arc::new(None.into()),
            human_voice_detection_context: Arc::new(None.into()),
        }
    }

    pub fn set_audio_tolerance(&self, tolerance: usize) -> Result<bool, String> {
        let mut tolerance_lock = self
            .audio_tolerance
            .write()
            .map_err(|err| format!("Failed to lock audio tolerance: {}", err))?;
        *tolerance_lock = tolerance;
        Ok(true)
    }

    pub fn set_microphone_gain(&self, microphone_gain: f32) -> Result<bool, String> {
        let mut microphone_gain_lock = self
            .microphone_gain
            .write()
            .map_err(|err| format!("Failed to lock microphone gain: {}", err))?;
        *microphone_gain_lock = microphone_gain;
        Ok(true)
    }

    pub fn set_speech_threshold(&self, speech_threshold: f32) -> Result<bool, String> {
        let mut speech_threshold_lock = self
            .speech_threshold
            .write()
            .map_err(|err| format!("Failed to lock speech threshold: {}", err))?;
        *speech_threshold_lock = speech_threshold;
        Ok(true)
    }

    pub fn set_speech_merge_threshold(&self, speech_merge_threshold: f32) -> Result<bool, String> {
        let mut speech_merge_threshold_lock = self
            .speech_merge_threshold
            .write()
            .map_err(|err| format!("Failed to lock speech merge threshold: {}", err))?;
        *speech_merge_threshold_lock = speech_merge_threshold;
        Ok(true)
    }

    pub fn set_recording_context(&self, recording_context: AudioContext) -> Result<(), String> {
        let mut recording_context_lock = self
            .recording_context
            .lock()
            .map_err(|err| format!("Failed to lock recording context: {}", err))?;
        recording_context_lock.replace(recording_context);
        Ok(())
    }

    pub fn set_human_voice_detection_context(
        &self,
        human_voice_detection_context: AudioContext,
    ) -> Result<(), String> {
        let mut human_voice_detection_context_lock = self
            .human_voice_detection_context
            .lock()
            .map_err(|err| format!("Failed to lock human voice detection context: {}", err))?;
        human_voice_detection_context_lock.replace(human_voice_detection_context);
        Ok(())
    }

    pub fn get_audio_tolerance(&self) -> Arc<RwLock<usize>> {
        self.audio_tolerance.clone()
    }

    pub fn get_microphone_gain(&self) -> Arc<RwLock<f32>> {
        self.microphone_gain.clone()
    }

    pub fn get_speech_threshold(&self) -> Arc<RwLock<f32>> {
        self.speech_threshold.clone()
    }

    pub fn get_speech_merge_threshold(&self) -> Arc<RwLock<f32>> {
        self.speech_merge_threshold.clone()
    }

    pub fn get_recording_context(&self) -> Arc<Mutex<Option<AudioContext>>> {
        self.recording_context.clone()
    }

    pub fn get_human_voice_detection_context(&self) -> Arc<Mutex<Option<AudioContext>>> {
        self.human_voice_detection_context.clone()
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_audio_tolerance(app_state: State<'_, AppState>, audio_tolerance: usize) -> Result<bool, String> {
    app_state.set_audio_tolerance(audio_tolerance)
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_microphone_gain(
    app_state: State<'_, AppState>,
    microphone_gain: f32,
) -> Result<bool, String> {
    app_state.set_microphone_gain(microphone_gain)
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_speech_threshold(
    app_state: State<'_, AppState>,
    speech_threshold: f32,
) -> Result<bool, String> {
    app_state.set_speech_threshold(speech_threshold)
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_speech_merge_threshold(
    app_state: State<'_, AppState>,
    speech_merge_threshold: f32,
) -> Result<bool, String> {
    app_state.set_speech_merge_threshold(speech_merge_threshold)
}
