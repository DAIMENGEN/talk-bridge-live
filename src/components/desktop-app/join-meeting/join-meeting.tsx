import "./join-meeting.scss";
import {Button, Input, Space} from "antd";
import {TranslateIcon01} from "@src/icons/translate/translate-icon-01.tsx";
import {useNavigate} from "react-router-dom";

export const JoinMeeting = () => {
    const navigate = useNavigate();
    return (
        <div className={"join-meeting"}>
            <Space direction={"vertical"} size={"large"}>
                <div className={"welcome-content-icon"}>
                    <TranslateIcon01 width={100} height={100} color={"#91003c"}/>
                </div>
                <Space direction={"vertical"} size={"middle"}>
                    <Space direction={"vertical"} className={"join-meeting-content-input"} size={"small"}>
                        <div className={"input-label"}>Meeting ID</div>
                        <Input placeholder={"Enter meeting ID"} defaultValue={"123456"}/>
                    </Space>
                    <Button shape="round" size={"middle"}>Next</Button>
                    <Button shape="round" size={"middle"} onClick={() => navigate("/welcome")}>Previous</Button>
                </Space>
            </Space>
        </div>
    )
}