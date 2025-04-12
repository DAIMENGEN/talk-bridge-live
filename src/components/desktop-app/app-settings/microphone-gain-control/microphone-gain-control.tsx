import {Slider} from "antd";
import "./microphone-gain-control.scss";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {setMicrophoneGain} from "@src/store/features/app-settings-slice.ts";
import {TauriDeviceService} from "@src/tauri-services/tauri-device-service.ts";
import {log} from "@src/logging.ts";

export const MicrophoneGainControl = () => {
    const appDispatch = useAppDispatch();
    const microphoneGain = useAppSelector((state) => state.appSettings.microphoneGain);
    return (
        <div className={"microphone-gain-control"}>
            <Slider
                min={0}
                max={3}
                step={0.1}
                onChange={(value) => {
                    TauriDeviceService.setMicrophoneGain(value).then((result) => {
                        if (result) {
                            appDispatch(setMicrophoneGain(value));
                        }
                    }).catch((error) => {
                        log.error("Set microphone gain error: ", error);
                    });
                }}
                defaultValue={microphoneGain}/>
        </div>
    )
}