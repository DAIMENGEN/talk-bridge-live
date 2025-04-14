import "./speech-threshold-adjust.scss";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {TauriStateService} from "@src/tauri-services/tauri-state-service.ts";
import {setSpeechThreshold} from "@src/store/features/app-settings-slice.ts";
import {log} from "@src/logger.ts";
import {Slider} from "antd";

export const SpeechThresholdAdjust = () => {
    const appDispatch = useAppDispatch();
    const speechThreshold = useAppSelector((state) => state.appSettings.speechThreshold);
    return (
        <div className={"speech-threshold-adjust"}>
            <Slider
                min={0}
                max={1}
                step={0.05}
                onChange={(value) => {
                    TauriStateService.setSpeechThreshold(value).then((result) => {
                        if (result) {
                            appDispatch(setSpeechThreshold(value));
                        }
                    }).catch((error) => {
                        log.error("Adjust speech threshold error: ", error);
                    });
                }}
                defaultValue={speechThreshold}/>
        </div>
    )
}