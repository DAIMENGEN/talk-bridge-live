import "./meeting-room.scss";
import {useParams} from "react-router-dom";
import {MoreIcon} from "@src/icons/more/more-icon.tsx";
import {FloatButton} from "antd";
import {SettingsFloatButton} from "@src/components/float-buttons";
import {useState} from "react";
import {DARK_MODE_BACKGROUND_COLOR} from "@src/theme/theme.ts";

export const MeetingRoom = () => {
    const {meetingId} = useParams();
    const [showFloatButtons, setShowFloatButtons] = useState(false);
    return (
        <div className={"meeting-room"}>
            <h1>Meeting ID: {meetingId}</h1>
            <FloatButton.Group trigger={"click"}
                               open={showFloatButtons}
                               onClick={() => setShowFloatButtons(v => !v)}
                               icon={<MoreIcon width={20} height={20} color={DARK_MODE_BACKGROUND_COLOR}/>}>
                <SettingsFloatButton/>
            </FloatButton.Group>
        </div>
    )
}