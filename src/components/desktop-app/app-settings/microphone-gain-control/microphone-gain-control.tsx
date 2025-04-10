import { Slider } from "antd";
import "./microphone-gain-control.scss";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {setMicrophoneGain} from "@src/store/features/app-settings-slice.ts";

export const MicrophoneGainControl = () => {
    const appDispatch = useAppDispatch();
    const microphoneGain = useAppSelector((state) => state.appSettings.microphoneGain);
    return (
        <div className={"microphone-gain-control"}>
            <Slider
                min={0}
                max={3}
                step={0.1}
                onChange={(value) => appDispatch(setMicrophoneGain(value))}
                defaultValue={microphoneGain}/>
        </div>
    )
}