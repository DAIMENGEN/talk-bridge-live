use crate::audio::audio_context::AudioContext;
use std::sync::{Arc, Mutex};
use tauri::State;

pub struct AppState {
    microphone_gain: Arc<Mutex<f32>>,
    recording_context: Arc<Mutex<Option<AudioContext>>>,
    human_voice_detection_context: Arc<Mutex<Option<AudioContext>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            microphone_gain: Arc::new(Mutex::new(1.0)),
            recording_context: Arc::new(None.into()),
            human_voice_detection_context: Arc::new(None.into()),
        }
    }

    pub fn set_microphone_gain(&self, microphone_gain: f32) -> Result<bool, String> {
        let mut microphone_gain_lock = self
            .microphone_gain
            .lock()
            .map_err(|err| format!("Failed to lock microphone gain: {}", err))?;
        *microphone_gain_lock = microphone_gain;
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

    pub fn set_human_voice_detection_context(&self, human_voice_detection_context: AudioContext) -> Result<(), String> {
        let mut human_voice_detection_context_lock = self
            .human_voice_detection_context
            .lock()
            .map_err(|err| format!("Failed to lock human voice detection context: {}", err))?;
        human_voice_detection_context_lock.replace(human_voice_detection_context);
        Ok(())
    }

    pub fn get_microphone_gain(&self) -> Arc<Mutex<f32>> {
        self.microphone_gain.clone()
    }

    pub fn get_recording_context(&self) -> Arc<Mutex<Option<AudioContext>>> {
        self.recording_context.clone()
    }

    pub fn get_human_voice_detection_context(&self) -> Arc<Mutex<Option<AudioContext>>> {
        self.human_voice_detection_context.clone()
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_microphone_gain(
    app_state: State<'_, AppState>,
    microphone_gain: f32,
) -> Result<bool, String> {
    app_state.set_microphone_gain(microphone_gain)
}
