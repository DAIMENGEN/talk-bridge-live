import "./speaker-select.scss";
import {Select} from "antd";
import {useEffect, useState} from "react";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {TauriDeviceService} from "@src/tauri-services/tauri-device-service.ts";
import {setSelectedSpeakerName} from "@src/store/features/app-settings-slice.ts";

export const SpeakerSelect = () => {
    const [speakerNames, setSpeakerNames] = useState<string[]>();
    const appDispatch = useAppDispatch();
    const selectedSpeakerName = useAppSelector((state) => state.appSettings.selectedSpeakerName);
    useEffect(() => {
        TauriDeviceService.listSpeakerNames().then(setSpeakerNames);
    }, []);
    return (
        <div className={"speaker-select"}>
            <Select
                defaultValue={selectedSpeakerName}
                placeholder={"Please select a speaker"}
                onChange={(value) => appDispatch(setSelectedSpeakerName(value))}
                options={speakerNames?.map((speakerName) => ({value: speakerName, label: speakerName}))}/>
        </div>
    )
}