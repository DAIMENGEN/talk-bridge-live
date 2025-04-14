import "./audio-tolerance-adjust.scss";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {TauriStateService} from "@src/tauri-services/tauri-state-service.ts";
import {setAudioTolerance} from "@src/store/features/app-settings-slice.ts";
import {log} from "@src/logger.ts";
import {Slider} from "antd";

export const AudioToleranceAdjust = () => {
    const appDispatch = useAppDispatch();
    const audioTolerance = useAppSelector((state) => state.appSettings.audioTolerance);
    return (
        <div className={"audio-tolerance-adjust"}>
            <Slider
                min={0}
                max={10}
                step={1}
                onChange={(value) => {
                    TauriStateService.setAudioTolerance(value).then((result) => {
                        if (result) {
                            appDispatch(setAudioTolerance(value));
                        }
                    }).catch((error) => {
                        log.error("Adjust audio tolerance error: ", error);
                    });
                }}
                defaultValue={audioTolerance}/>
        </div>
    )
}