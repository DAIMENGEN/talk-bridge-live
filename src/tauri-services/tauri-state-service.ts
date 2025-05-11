import {invoke} from "@tauri-apps/api/core";

export class TauriStateService {

    static setMicrophoneGain(microphoneGain: number): Promise<boolean> {
        return invoke<boolean>("set_microphone_gain", {microphone_gain: microphoneGain});
    }

    static setSpeechThreshold(speechThreshold: number): Promise<boolean> {
        return invoke<boolean>("set_speech_threshold", {speech_threshold: speechThreshold});
    }

    static setAudioGapThreshold(audioGapThreshold: number): Promise<boolean> {
        return invoke<boolean>("set_audio_gap_threshold", {audio_gap_threshold: audioGapThreshold});
    }

    static setSilenceStreakCount(silenceStreakCount: number): Promise<boolean> {
        return invoke<boolean>("set_silence_streak_count", {silence_streak_count: silenceStreakCount});
    }
}