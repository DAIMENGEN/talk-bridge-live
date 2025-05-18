import {invoke} from "@tauri-apps/api/core";

export class TauriStateService {

    static setMicrophoneGain(microphoneGain: number): Promise<boolean> {
        return invoke<boolean>("set_microphone_gain", {microphone_gain: microphoneGain});
    }

    static setSpeechThreshold(speechThreshold: number): Promise<boolean> {
        return invoke<boolean>("set_speech_threshold", {speech_threshold: speechThreshold});
    }

    static setSilenceStreakThreshold(silenceStreakThreshold: number): Promise<boolean> {
        return invoke<boolean>("set_silence_streak_threshold", {silence_streak_threshold: silenceStreakThreshold});
    }
}