import "./join-meeting.scss";
import {useState} from "react";
import {useNavigate} from "react-router-dom";
import {Button, Input, Space, message, FloatButton} from "antd";
import {LegalNotice} from "@src/components/desktop-app/legal-notice/legal-notice.tsx";
import {InternationalIcon, MoreIcon} from "@src/icons";
import {
    AppAboutFloatButton,
    CheckUpdateFloatButton,
    ReportIssueFloatButton,
    UserManualFloatButton
} from "@src/components/float-buttons";
import {ADVANTEST_COLOR} from "@src/theme/theme.ts";

export const JoinMeeting = () => {
    const navigate = useNavigate();
    const [meetingId, setMeetingId] = useState("");
    const [messageApi, contextHolder] = message.useMessage();
    return (
        <div className={"join-meeting"}>
            {contextHolder}
            <Space direction={"vertical"} size={"large"}>
                <div className={"welcome-content-icon"}>
                    <InternationalIcon width={100} height={100} color={ADVANTEST_COLOR}/>
                </div>
                <Space direction={"vertical"} size={"middle"}>
                    <Space direction={"vertical"} className={"join-meeting-content-input"} size={"small"}>
                        <div className={"input-label"}>Meeting ID</div>
                        <Input placeholder={"Enter meeting ID"} onChange={e => setMeetingId(e.target.value)} defaultValue={meetingId}/>
                    </Space>
                    <Button className={"next-button"} shape="round" size={"middle"} onClick={async () => {
                        if (meetingId.length === 0) {
                            await messageApi.warning("Please enter a meeting ID");
                            return;
                        }
                        navigate("/meeting-room/" + meetingId);
                    }}>Next</Button>
                    <Button className={"previous-button"} shape="round" size={"middle"} onClick={() => navigate("/welcome")}>Previous</Button>
                </Space>
            </Space>
            <LegalNotice/>
            <FloatButton.Group trigger={"click"}
                               icon={<MoreIcon width={20} height={20} color={"#141414"}/>}>
                <UserManualFloatButton/>
                <ReportIssueFloatButton/>
                <AppAboutFloatButton/>
                <CheckUpdateFloatButton/>
            </FloatButton.Group>
        </div>
    )
}