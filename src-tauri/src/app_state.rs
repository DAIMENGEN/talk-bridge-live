use crate::audio::context::AudioContext;
use std::sync::{Arc, Mutex, RwLock};
use tauri::State;

pub const DEFAULT_TOLERANCE: usize = 1;

pub const DEFAULT_MICROPHONE_GAIN: f32 = 1.0;

pub const DEFAULT_SPEECH_THRESHOLD: f32 = 0.5;

pub const DEFAULT_AUDIO_GAP_THRESHOLD: f32 = 0.5;

pub const DEFAULT_ASR_SERVICE_URL: &str = "http://10.150.112.34:50051";

pub struct AppState {
    speaker: Arc<RwLock<String>>,
    meeting_room: Arc<RwLock<String>>,
    asr_service_url: Arc<RwLock<String>>,
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
            speaker: Arc::new(RwLock::new(whoami::username())),
            meeting_room: Arc::new(RwLock::new(whoami::fallible::hostname().unwrap_or(whoami::username()))),
            asr_service_url: Arc::new(RwLock::new(DEFAULT_ASR_SERVICE_URL.to_string())),
            audio_tolerance: Arc::new(RwLock::new(DEFAULT_TOLERANCE)),
            microphone_gain: Arc::new(RwLock::new(DEFAULT_MICROPHONE_GAIN)),
            speech_threshold: Arc::new(RwLock::new(DEFAULT_SPEECH_THRESHOLD)),
            speech_merge_threshold: Arc::new(RwLock::new(DEFAULT_AUDIO_GAP_THRESHOLD)),
            recording_context: Arc::new(None.into()),
            human_voice_detection_context: Arc::new(None.into()),
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
    
    pub fn set_asr_service_url(&self, asr_service_url: String) -> Result<bool, String> {
        let mut asr_service_url_lock = self
            .asr_service_url
            .write()
            .map_err(|err| format!("Failed to lock ASR service URL: {}", err))?;
        *asr_service_url_lock = asr_service_url;
        Ok(true)
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

    pub fn get_speaker(&self) -> Arc<RwLock<String>> {
        self.speaker.clone()
    }

    pub fn get_meeting_room(&self) -> Arc<RwLock<String>> {
        self.meeting_room.clone()
    }
    
    pub fn get_asr_service_url(&self) -> Arc<RwLock<String>> {
        self.asr_service_url.clone()
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
pub fn set_speaker(app_state: State<'_, AppState>, speaker: String) -> Result<bool, String> {
    app_state.set_speaker(speaker)
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_meeting_room(app_state: State<'_, AppState>, meeting_room: String) -> Result<bool, String> {
    app_state.set_meeting_room(meeting_room)
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_asr_service_url(app_state: State<'_, AppState>, asr_service_url: String) -> Result<bool, String> {
    app_state.set_asr_service_url(asr_service_url)
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
