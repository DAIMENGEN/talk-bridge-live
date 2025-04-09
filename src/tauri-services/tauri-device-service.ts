import {invoke} from "@tauri-apps/api/core";

export class TauriDeviceService {
    static listSpeakerNames(): Promise<string[]> {
        return invoke<string[]>("list_speaker_names");
    }
    static listMicrophoneNames(): Promise<string[]> {
        return invoke<string[]>("list_microphone_names");
    }
}