import {invoke} from "@tauri-apps/api/core";

export class TauriDeviceService {
    static humanVoiceDetection(deviceName: string): Promise<string> {
        return invoke<string>("human_voice_detection", {device_name: deviceName});
    }

    static stopHumanVoiceDetection(): Promise<boolean> {
        return invoke<boolean>("stop_human_voice_detection");
    }

    static listSpeakerNames(): Promise<string[]> {
        return invoke<string[]>("list_speaker_names");
    }

    static listMicrophoneNames(): Promise<string[]> {
        return invoke<string[]>("list_microphone_names");
    }

    static setMicrophoneGain(microphoneGain: number): Promise<boolean> {
        return invoke<boolean>("set_microphone_gain", {microphone_gain: microphoneGain});
    }
}