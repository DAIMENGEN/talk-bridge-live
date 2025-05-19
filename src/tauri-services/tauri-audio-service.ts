import {invoke} from "@tauri-apps/api/core";

export class TauriAudioService {

    static startASR(deviceName: string): Promise<string> {
        return invoke("start_asr", { microphone_name: deviceName });
    }

    static stopASR(): Promise<boolean> {
        return invoke<boolean>("stop_asr");
    }
}