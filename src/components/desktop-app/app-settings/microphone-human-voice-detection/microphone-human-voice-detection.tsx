import "./microphone-human-voice-detection.scss";
import {log} from "@src/logging.ts";
import {useEffect, useState} from "react";
import {useAppSelector} from "@src/store/store.ts";
import {TauriDeviceService} from "@src/tauri-services/tauri-device-service.ts";
import {TauriService} from "@src/tauri-services/tauri-service.ts";
import {UnlistenFn} from "@tauri-apps/api/event";

interface HumanVoiceProbability {
    probability: number;
}

export const MicrophoneHumanVoiceDetection = () => {
    const [probability, setProbability] = useState<number>(0);
    const microphoneName = useAppSelector((state) => state.appSettings.selectedMicrophoneName);

    useEffect(() => {
        let unlisten: UnlistenFn;
        if (microphoneName) {
            TauriDeviceService.humanVoiceDetection(microphoneName).then(eventName => {
                TauriService.listen<HumanVoiceProbability>(eventName, (event) => {
                    setProbability(event.payload.probability);
                }).then(result => {
                    unlisten = result;
                }).catch(error => {
                    log.error("Listen human voice detection event error:", error)
                });
            });
        }
        return () => {
            unlisten && unlisten();
            TauriDeviceService.stopHumanVoiceDetection().then(result => log.debug("Stop human voice detection: ", result));
        }
    }, [microphoneName]);

    return (
        <div className={"microphone-volume-bars"}>
            {Array.from({length: 100}).map((_, i) => (
                <div
                    key={i}
                    className={`mic-bar ${i < probability * 100 ? "active" : ""}`}
                />
            ))}
        </div>
    )
}