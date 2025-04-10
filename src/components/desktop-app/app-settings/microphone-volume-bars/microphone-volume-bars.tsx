import "./microphone-volume-bars.scss";
import {useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import {useAppSelector} from "@src/store/store.ts";
import {invoke} from "@tauri-apps/api/core";

export const MicrophoneVolumeBars = () => {
    const [volumeLevel, setVolumeLevel] = useState<number>(0);
    const microphoneName = useAppSelector((state) => state.appSettings.selectedMicrophoneName);

    useEffect(() => {
        listen<{ value: number }>("microphone_realtime_volume", (event) => {
            console.log("microphone_realtime_volume", event.payload.value);
            setVolumeLevel(event.payload.value);
        }).then();
    }, []);

    useEffect(() => {
        if (microphoneName) {
            invoke("start_microphone_test", {device_name: microphoneName}).then();
        }
        return () => {
            invoke("stop_microphone_test").then();
        }
    }, [microphoneName]);

    return (
        <div className={"microphone-volume-bars"}>
            {Array.from({length: 100}).map((_, i) => (
                <div
                    key={i}
                    className={`mic-bar ${i < volumeLevel ? "active" : ""}`}
                />
            ))}
        </div>
    )
}