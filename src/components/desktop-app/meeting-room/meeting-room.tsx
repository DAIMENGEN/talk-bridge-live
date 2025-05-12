import "./meeting-room.scss";
import {useParams} from "react-router-dom";
import {MoreIcon} from "@src/icons/more/more-icon.tsx";
import {FloatButton} from "antd";
import {SettingsFloatButton} from "@src/components/float-buttons";

export const MeetingRoom = () => {
    const { meetingId } = useParams();
    return (
        <div className={"meeting-room"}>
            <h1>Meeting ID: {meetingId}</h1>
            <FloatButton.Group trigger={"click"}
                               icon={<MoreIcon width={20} height={20} color={"#141414"}/>}>
                <SettingsFloatButton/>
            </FloatButton.Group>
        </div>
    )
}