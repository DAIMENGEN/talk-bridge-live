import "./meeting-room.scss";
import {useState} from "react";
import {FloatButton} from "antd";
import {MoreIcon} from "@src/icons/more/more-icon.tsx";
import {DARK_MODE_BACKGROUND_COLOR} from "@src/theme/theme.ts";
import {VoiceActivityVisualizer} from "@src/components/common";
import {MicrophoneFloatButton, SettingsFloatButton} from "@src/components/float-buttons";

export const MeetingRoom = () => {
    // const {meetingId} = useParams();
    const [showFloatButtons, setShowFloatButtons] = useState(false);
    return (
        <div className={"meeting-room"}>
            <FloatButton.Group trigger={"click"}
                               open={showFloatButtons}
                               onClick={() => setShowFloatButtons(v => !v)}
                               icon={<MoreIcon width={20} height={20} color={DARK_MODE_BACKGROUND_COLOR}/>}>
                <MicrophoneFloatButton/>
                <SettingsFloatButton/>
            </FloatButton.Group>
            <div className={"voice-activity-visualizer-container"}>
                <VoiceActivityVisualizer/>
            </div>
        </div>
    )
}