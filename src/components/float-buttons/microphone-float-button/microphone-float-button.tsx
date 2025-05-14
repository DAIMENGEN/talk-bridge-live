import "./microphone-float-button.scss";
import {FloatButton} from "antd";
import {DARK_MODE_BACKGROUND_COLOR} from "@src/theme/theme.ts";
import {MicrophoneOffIcon, MicrophoneOnIcon} from "@src/icons";
import {useState} from "react";
export const MicrophoneFloatButton = () => {
    const [disabled, setDisabled] = useState(false);
    return (
        <FloatButton icon={disabled ? <MicrophoneOnIcon width={20} height={20} color={DARK_MODE_BACKGROUND_COLOR}/> : <MicrophoneOffIcon width={20} height={20} color={"#141414"}/>}
                     tooltip={disabled ? "Disable the microphone" : "Enable the microphone"}
                     onClick={() => setDisabled(disabled => !disabled)}/>
    )
}