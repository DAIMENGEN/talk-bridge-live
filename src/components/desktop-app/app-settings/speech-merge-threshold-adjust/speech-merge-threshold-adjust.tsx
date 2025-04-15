import "./speech-merge-threshold-adjust.scss";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {TauriStateService} from "@src/tauri-services/tauri-state-service.ts";
import {setSpeechMergeThreshold} from "@src/store/features/app-settings-slice.ts";
import {log} from "@src/logger.ts";
import {Slider} from "antd";

export const SpeechMergeThresholdAdjust = () => {
    const appDispatch = useAppDispatch();
    const speechMergeThreshold = useAppSelector((state) => state.appSettings.speechMergeThreshold);
    return (
        <div className={"speech-merge-threshold-adjust"}>
            <Slider
                min={0}
                max={2}
                step={0.05}
                onChange={(value) => {
                    TauriStateService.setSpeechMergeThreshold(value).then((result) => {
                        if (result) {
                            appDispatch(setSpeechMergeThreshold(value));
                        }
                    }).catch((error) => {
                        log.error("Adjust speech merge threshold error: ", error);
                    });
                }}
                defaultValue={speechMergeThreshold}/>
        </div>
    )
}