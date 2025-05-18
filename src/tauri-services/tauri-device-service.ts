import {invoke} from "@tauri-apps/api/core";

export class TauriDeviceService {
    static testMicrophone(deviceName: string): Promise<string> {
        return invoke<string>("test_microphone", {device_name: deviceName});
    }

    static stopTestMicrophone(): Promise<void> {
        return invoke<void>("stop_test_microphone");
    }

    static listSpeakerNames(): Promise<string[]> {
        return invoke<string[]>("list_speaker_names");
    }

    static listMicrophoneNames(): Promise<string[]> {
        return invoke<string[]>("list_microphone_names");
    }
}