import "./join-meeting.scss";
import {useState} from "react";
import {useNavigate} from "react-router-dom";
import {Button, Input, Space, message } from "antd";
import {InternationalIcon} from "@src/icons/international/international-icon.tsx";
import {LegalNotice} from "@src/components/desktop-app/legal-notice/legal-notice.tsx";

export const JoinMeeting = () => {
    const navigate = useNavigate();
    const [meetingId, setMeetingId] = useState("");
    const [messageApi, contextHolder] = message.useMessage();
    return (
        <div className={"join-meeting"}>
            <Space direction={"vertical"} size={"large"}>
                <div className={"welcome-content-icon"}>
                    <InternationalIcon width={100} height={100} color={"#91003c"}/>
                </div>
                <Space direction={"vertical"} size={"middle"}>
                    <Space direction={"vertical"} className={"join-meeting-content-input"} size={"small"}>
                        <div className={"input-label"}>Meeting ID</div>
                        <Input placeholder={"Enter meeting ID"} onChange={e => setMeetingId(e.target.value)} defaultValue={meetingId}/>
                    </Space>
                    <Button shape="round" size={"middle"} onClick={async () => {
                        if (meetingId.length === 0) {
                            await messageApi.warning("Please enter a meeting ID");
                            return;
                        }
                    }}>Next</Button>
                    <Button shape="round" size={"middle"} onClick={() => navigate("/welcome")}>Previous</Button>
                </Space>
            </Space>
            <LegalNotice/>
            {contextHolder}
        </div>
    )
}