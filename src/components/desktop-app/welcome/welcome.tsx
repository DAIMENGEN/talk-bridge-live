import "./welcome.scss";
import {Toolbar} from "@src/components/desktop-app/toolbar/toolbar.tsx";
import {Button, Space} from "antd";

export const Welcome = () => {

    return (
        <div className={"desktop-welcome"}>
            <div className={"frosted-glass"}>
                <Toolbar/>
                <Space className={"desktop-welcome-content"} direction="vertical" size="small">
                    <h1>Talk Bridge Live</h1>
                    <h1>AI real-time translation</h1>
                    <h5>
                        Talk Bridge Live is a real-time translation platform that allows you to translate your live
                        streams in real-time.
                    </h5>
                    <div className={"desktop-welcome-content-button"}>
                        <Button size={"large"}>开始使用</Button>
                    </div>
                </Space>
            </div>
        </div>
    )
}