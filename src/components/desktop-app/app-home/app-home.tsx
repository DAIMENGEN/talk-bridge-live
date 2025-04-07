import "./app-home.scss";
import { listen } from "@tauri-apps/api/event";
import {useEffect, useState} from "react";
import {TauriDeviceService} from "@src/tauri-services/tauri-device-service.ts";
import {Button, Select} from "antd";
import {invoke} from "@tauri-apps/api/core";

type TranscriptionData = {
    text: string;
}
export const AppHome = () => {

    const [texts, setTexts] = useState<TranscriptionData[]>();

    const [microphoneNames, setMicrophoneNames] = useState<string[]>();

    const [targetMicrophoneName, setTargetMicrophoneName] = useState<string>();

    useEffect(() => {
        listen<TranscriptionData>("start_recording", (event) => {
            console.log(event.payload.text);
            setTexts((prev) => [...(prev ?? []), event.payload]);
        }).then();
    }, []);

    useEffect(() => {
        TauriDeviceService.listMicrophoneNames().then((microphoneNames) => {
            setMicrophoneNames(microphoneNames);
        });
    }, []);

    return (
        <div className={"app-home"}>

            <div>
                <Select
                    defaultValue="lucy"
                    style={{ width: 120 }}
                    onChange={(value) => {
                        setTargetMicrophoneName(value);
                    }}
                    options={microphoneNames?.map((microphoneName) => ({value: microphoneName, label: microphoneName}))}
                />
            </div>

            <div>
                <Button type="text" onClick={async () => {
                    await invoke("start_recording", {device_name: targetMicrophoneName});
                }}>开始录音</Button>
                <Button type="text" onClick={async () => {
                    await invoke("stop_recording");
                }}>停止录音</Button>
            </div>

            <div style={{marginTop: "30px", color: "white"}}>
                {texts?.map((text, index) => (
                    <div key={index}>{text.text}</div>
                ))}
            </div>
        </div>
    )
}