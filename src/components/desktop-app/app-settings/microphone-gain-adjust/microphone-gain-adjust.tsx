import {Slider} from "antd";
import "./microphone-gain-adjust.scss";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {setMicrophoneGain} from "@src/store/features/app-settings-slice.ts";
import {log} from "@src/logger.ts";
import {TauriStateService} from "@src/tauri-services/tauri-state-service.ts";

export const MicrophoneGainAdjust = () => {
    const appDispatch = useAppDispatch();
    const microphoneGain = useAppSelector((state) => state.appSettings.microphoneGain);
    return (
        <div className={"microphone-gain-adjust"}>
            <Slider
                min={0}
                max={3}
                step={0.1}
                onChange={(value) => {
                    TauriStateService.setMicrophoneGain(value).then((result) => {
                        if (result) {
                            appDispatch(setMicrophoneGain(value));
                        }
                    }).catch((error) => {
                        log.error("Adjust microphone gain error: ", error);
                    });
                }}
                defaultValue={microphoneGain}/>
        </div>
    )
}