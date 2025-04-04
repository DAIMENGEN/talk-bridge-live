import {invoke} from "@tauri-apps/api/core";

export class TauriAudioRecorderService {

    static startRecording(deviceName: string): Promise<boolean> {
        return invoke("start_recording", { device_name: deviceName })
    }

    static stopRecording(): Promise<boolean> {
        return invoke<boolean>("stop_recording")
    }
}