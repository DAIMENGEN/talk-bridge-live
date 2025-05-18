use crate::audio::context::microphone_context::MicrophoneContext;
use std::sync::{Arc, Mutex, RwLock};
use tauri::State;

pub const DEFAULT_MICROPHONE_GAIN: f32 = 1.0;

pub const DEFAULT_SPEECH_THRESHOLD: f32 = 0.75;

pub const DEFAULT_SILENCE_STREAK_THRESHOLD: usize = 3;

pub struct AppState {
    speaker: Arc<RwLock<String>>,
    meeting_room: Arc<RwLock<String>>,
    microphone_gain: Arc<RwLock<f32>>,
    speech_threshold: Arc<RwLock<f32>>,
    silence_streak_threshold: Arc<RwLock<usize>>,
    microphone_context: Arc<Mutex<Option<MicrophoneContext>>>,
    microphone_context_test: Arc<Mutex<Option<MicrophoneContext>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            speaker: Arc::new(RwLock::new(whoami::username())),
            meeting_room: Arc::new(RwLock::new(
                whoami::fallible::hostname().unwrap_or(whoami::username()),
            )),
            microphone_gain: Arc::new(RwLock::new(DEFAULT_MICROPHONE_GAIN)),
            speech_threshold: Arc::new(RwLock::new(DEFAULT_SPEECH_THRESHOLD)),
            silence_streak_threshold: Arc::new(RwLock::new(DEFAULT_SILENCE_STREAK_THRESHOLD)),
            microphone_context: Arc::new(None.into()),
            microphone_context_test: Arc::new(None.into()),
        }
    }

    pub fn set_speaker(&self, speaker: String) -> Result<bool, String> {
        let mut speaker_lock = self
            .speaker
            .write()
            .map_err(|err| format!("Failed to lock speaker: {}", err))?;
        *speaker_lock = speaker;
        Ok(true)
    }

    pub fn set_meeting_room(&self, meeting_room: String) -> Result<bool, String> {
        let mut meeting_room_lock = self
            .meeting_room
            .write()
            .map_err(|err| format!("Failed to lock meeting room: {}", err))?;
        *meeting_room_lock = meeting_room;
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

    pub fn set_silence_streak_threshold(
        &self,
        silence_streak_threshold: usize,
    ) -> Result<bool, String> {
        let mut silence_streak_threshold_lock = self
            .silence_streak_threshold
            .write()
            .map_err(|err| format!("Failed to lock silence streak threshold: {}", err))?;
        *silence_streak_threshold_lock = silence_streak_threshold;
        Ok(true)
    }

    pub fn set_microphone_context(
        &self,
        microphone_context: MicrophoneContext,
    ) -> Result<(), String> {
        let mut microphone_context_lock = self
            .microphone_context
            .lock()
            .map_err(|err| format!("Failed to lock microphone context: {}", err))?;
        microphone_context_lock.replace(microphone_context);
        Ok(())
    }

    pub fn set_microphone_context_test(
        &self,
        microphone_context_test: MicrophoneContext,
    ) -> Result<(), String> {
        let mut microphone_context_test_lock = self
            .microphone_context_test
            .lock()
            .map_err(|err| format!("Failed to lock microphone context test: {}", err))?;
        microphone_context_test_lock.replace(microphone_context_test);
        Ok(())
    }

    pub fn get_speaker(&self) -> Arc<RwLock<String>> {
        self.speaker.clone()
    }

    pub fn get_meeting_room(&self) -> Arc<RwLock<String>> {
        self.meeting_room.clone()
    }

    pub fn get_microphone_gain(&self) -> Arc<RwLock<f32>> {
        self.microphone_gain.clone()
    }

    pub fn get_speech_threshold(&self) -> Arc<RwLock<f32>> {
        self.speech_threshold.clone()
    }

    pub fn get_silence_streak_threshold(&self) -> Arc<RwLock<usize>> {
        self.silence_streak_threshold.clone()
    }

    pub fn get_microphone_context(&self) -> Arc<Mutex<Option<MicrophoneContext>>> {
        self.microphone_context.clone()
    }

    pub fn get_microphone_context_test(&self) -> Arc<Mutex<Option<MicrophoneContext>>> {
        self.microphone_context_test.clone()
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_speaker(app_state: State<'_, AppState>, speaker: String) -> Result<bool, String> {
    app_state.set_speaker(speaker)
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_meeting_room(
    app_state: State<'_, AppState>,
    meeting_room: String,
) -> Result<bool, String> {
    app_state.set_meeting_room(meeting_room)
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
pub fn set_silence_streak_threshold(
    app_state: State<'_, AppState>,
    silence_streak_threshold: usize,
) -> Result<bool, String> {
    app_state.set_silence_streak_threshold(silence_streak_threshold)
}
