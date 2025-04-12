import "./speaker-human-voice-detection.scss";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {setSpeakerVolume} from "@src/store/features/app-settings-slice.ts";
import {Slider} from "antd";

export const SpeakerHumanVoiceDetection = () => {
    const appDispatch = useAppDispatch();
    const speakerVolume = useAppSelector((state) => state.appSettings.speakerVolume);

    return (
        <div className={"speaker-human-voice-detection"}>
            <Slider
                min={0}
                max={3}
                step={0.1}
                onChange={(value) => appDispatch(setSpeakerVolume(value))}
                defaultValue={speakerVolume}/>
        </div>
    )
}