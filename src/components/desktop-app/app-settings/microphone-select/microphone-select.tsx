import "./microphone-select.scss";
import {useEffect, useState} from "react";
import {TauriDeviceService} from "@src/tauri-services/tauri-device-service.ts";
import {Select} from "antd";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {setSelectedMicrophoneName} from "@src/store/features/app-settings-slice.ts";

export const MicrophoneSelect = () => {
    const [microphoneNames, setMicrophoneNames] = useState<string[]>();
    const appDispatch = useAppDispatch();
    const selectedMicrophoneName = useAppSelector((state) => state.appSettings.selectedMicrophoneName);
    useEffect(() => {
        TauriDeviceService.listMicrophoneNames().then(setMicrophoneNames);
    }, []);
    return (
        <div className={"microphone-select"}>
            <Select
                defaultValue={selectedMicrophoneName}
                placeholder={"Please select a microphone"}
                onChange={(value) => appDispatch(setSelectedMicrophoneName(value))}
                options={microphoneNames?.map((microphoneName) => ({value: microphoneName, label: microphoneName}))}/>
        </div>
    )
}