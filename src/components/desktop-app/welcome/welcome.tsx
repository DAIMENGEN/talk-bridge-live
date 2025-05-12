import "./welcome.scss";
import {Button, Space} from "antd";
import {useNavigate} from "react-router-dom";
import {InternationalIcon} from "@src/icons/international/international-icon.tsx";
import {LegalNotice} from "@src/components/desktop-app/legal-notice/legal-notice.tsx";

export const Welcome = () => {
    const navigate = useNavigate();
    return (
        <div className={"welcome"}>
            <Space direction={"vertical"} size={"large"}>
                <div className={"welcome-content-icon"}>
                    <InternationalIcon width={100} height={100} color={"#91003c"}/>
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
            <LegalNotice/>
        </div>
    )
}