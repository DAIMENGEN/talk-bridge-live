import "./welcome.scss";
import {Button, Space} from "antd";
import {TranslateIcon01} from "@src/icons/translate/translate-icon-01.tsx";
import {useNavigate} from "react-router-dom";

export const Welcome = () => {
    const navigate = useNavigate();
    return (
        <div className={"welcome"}>
            <Space direction={"vertical"} size={"large"}>
                <div className={"welcome-content-icon"}>
                    <TranslateIcon01 width={100} height={100} color={"#91003c"}/>
                </div>
                <div className={"welcome-content-title"}>
                    Talk Bridge Live
                </div>
                <Space direction={"vertical"} size={"small"}>
                    <div className={"welcome-content-button"}>
                        <Button shape="round" size={"middle"} onClick={() => navigate("/join-meeting")}>Join Meeting</Button>
                    </div>
                </Space>
            </Space>
        </div>
    )
}