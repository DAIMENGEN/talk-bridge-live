import "./welcome.scss";
import {Button, FloatButton, Space} from "antd";
import {useNavigate} from "react-router-dom";
import {LegalNotice} from "@src/components/desktop-app/legal-notice/legal-notice.tsx";
import {InternationalIcon, MoreIcon} from "@src/icons";
import {
    AppAboutFloatButton,
    CheckUpdateFloatButton,
    ReportIssueFloatButton,
    UserManualFloatButton
} from "@src/components/float-buttons";
import {ADVANTEST_COLOR, DARK_MODE_BACKGROUND_COLOR} from "@src/theme/theme";

export const Welcome = () => {
    const navigate = useNavigate();
    return (
        <div className={"welcome"}>
            <Space direction={"vertical"} size={"large"}>
                <div className={"welcome-content-icon"}>
                    <InternationalIcon width={100} height={100} color={ADVANTEST_COLOR}/>
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
            <FloatButton.Group trigger={"click"}
                               icon={<MoreIcon width={20} height={20} color={DARK_MODE_BACKGROUND_COLOR}/>}>
                <UserManualFloatButton/>
                <ReportIssueFloatButton/>
                <AppAboutFloatButton/>
                <CheckUpdateFloatButton/>
            </FloatButton.Group>
        </div>
    )
}