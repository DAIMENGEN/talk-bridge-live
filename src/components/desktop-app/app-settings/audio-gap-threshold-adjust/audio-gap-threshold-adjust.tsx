import "./audio-gap-threshold-adjust.scss";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {TauriStateService} from "@src/tauri-services/tauri-state-service.ts";
import {setAudioGapThreshold} from "@src/store/features/app-settings-slice.ts";
import {log} from "@src/logger.ts";
import {Slider} from "antd";

export const AudioGapThresholdAdjust = () => {
    const appDispatch = useAppDispatch();
    const audioGapThreshold = useAppSelector((state) => state.appSettings.audioGapThreshold);
    return (
        <div className={"audio-gap-threshold-adjust"}>
            <Slider
                min={0}
                max={2}
                step={0.05}
                onChange={(value) => {
                    TauriStateService.setSpeechMergeThreshold(value).then((result) => {
                        if (result) {
                            appDispatch(setAudioGapThreshold(value));
                        }
                    }).catch((error) => {
                        log.error("Adjust audio gap threshold error: ", error);
                    });
                }}
                defaultValue={audioGapThreshold}/>
        </div>
    )
}