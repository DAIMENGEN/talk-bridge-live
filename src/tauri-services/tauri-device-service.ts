import {invoke} from "@tauri-apps/api/core";

export class TauriDeviceService {

    static listMicrophoneNames(): Promise<string[]> {
        return invoke<string[]>("list_microphone_names");
    }
}