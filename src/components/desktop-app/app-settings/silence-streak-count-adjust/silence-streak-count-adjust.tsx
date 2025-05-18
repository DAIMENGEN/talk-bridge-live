import "./silence-streak-count-adjust.scss";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {TauriStateService} from "@src/tauri-services/tauri-state-service.ts";
import {setSilenceStreakCount} from "@src/store/features/app-settings-slice.ts";
import {log} from "@src/logger.ts";
import {Slider} from "antd";

export const SilenceStreakCountAdjust = () => {
    const appDispatch = useAppDispatch();
    const silenceStreakCount = useAppSelector((state) => state.appSettings.silenceStreakCount);
    return (
        <div className={"silence-streak-count-adjust"}>
            <Slider
                min={0}
                max={10}
                step={1}
                onChange={(value) => {
                    TauriStateService.setSilenceStreakThreshold(value).then((result) => {
                        if (result) {
                            appDispatch(setSilenceStreakCount(value));
                        }
                    }).catch((error) => {
                        log.error("Adjust silence streak count error: ", error);
                    });
                }}
                defaultValue={silenceStreakCount}/>
        </div>
    )
}