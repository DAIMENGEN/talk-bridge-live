import "./welcome.scss";
import {Button, Space} from "antd";
import {TranslateIcon01} from "@src/icons/translate/translate-icon-01.tsx";

export const Welcome = () => {
    return (
        <div className={"desktop-welcome"}>
            <Space direction={"vertical"} size={"large"}>
                <div className={"desktop-welcome-content-icon"}>
                    <TranslateIcon01 width={100} height={100} color={"#91003c"}/>
                </div>
                <div className={"desktop-welcome-content-title"}>
                    Talk Bridge Live
                </div>
                <Space direction={"vertical"} size={"small"}>
                    <div className={"desktop-welcome-content-button"}>
                        <Button shape="round" size={"middle"}>Join Meeting</Button>
                    </div>
                </Space>
            </Space>
        </div>
    )
}