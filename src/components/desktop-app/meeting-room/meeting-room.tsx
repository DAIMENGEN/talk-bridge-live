import "./meeting-room.scss";
import {useState} from "react";
import {FloatButton, Space} from "antd";
import {MoreIcon} from "@src/icons/more/more-icon.tsx";
import {DARK_MODE_BACKGROUND_COLOR} from "@src/theme/theme.ts";
import {VoiceActivityVisualizer} from "@src/components/common";
import {MicrophoneFloatButton, SettingsFloatButton} from "@src/components/float-buttons";
import {useAppSelector} from "@src/store/store.ts";
import {UtteranceContent} from "@src/components/desktop-app/utterance-content/utterance-content.tsx";

export const MeetingRoom = () => {
    // const {meetingId} = useParams();
    const [showFloatButtons, setShowFloatButtons] = useState(false);
    const utterances = useAppSelector(state => state.meetingContent.utterances);
    return (
        <div className={"meeting-room"}>
            <FloatButton.Group trigger={"click"}
                               open={showFloatButtons}
                               onClick={() => setShowFloatButtons(v => !v)}
                               icon={<MoreIcon width={20} height={20} color={DARK_MODE_BACKGROUND_COLOR}/>}>
                <MicrophoneFloatButton/>
                <SettingsFloatButton/>
            </FloatButton.Group>
            <Space className={"meeting-room-content"} direction="vertical" size={"small"}>
                {
                    utterances.map((utterance, index) => (
                        <UtteranceContent key={index} {...utterance}/>
                    ))
                }
            </Space>
            <div className={"voice-activity-visualizer-container"}>
                <VoiceActivityVisualizer/>
            </div>
        </div>
    )
}