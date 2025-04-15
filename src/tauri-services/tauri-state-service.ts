import {invoke} from "@tauri-apps/api/core";

export class TauriStateService {

    static setAudioTolerance(tolerance: number): Promise<boolean> {
        return invoke<boolean>("set_audio_tolerance", {audio_tolerance: tolerance});
    }

    static setMicrophoneGain(microphoneGain: number): Promise<boolean> {
        return invoke<boolean>("set_microphone_gain", {microphone_gain: microphoneGain});
    }

    static setSpeechThreshold(speechThreshold: number): Promise<boolean> {
        return invoke<boolean>("set_speech_threshold", {speech_threshold: speechThreshold});
    }

    static setSpeechMergeThreshold(speechMergeThreshold: number): Promise<boolean> {
        return invoke<boolean>("set_speech_merge_threshold", {speech_merge_threshold: speechMergeThreshold});
    }
}