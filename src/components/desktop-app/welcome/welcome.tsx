import "./welcome.scss";
import {Button, Space} from "antd";

export const Welcome = () => {
    return (
        <div className={"desktop-welcome"}>
            <div className={"desktop-welcome-content"}>
                <Space direction="vertical" size="small">
                    <h1>Talk Bridge Live</h1>
                    <h2>AI Simultaneous Interpretation – Breaking Language Barriers</h2>
                    <h3>
                        <span>
                            Talk Bridge Live is an AI simultaneous translation software meticulously developed by Advantest China R&D, designed for corporate meetings, international exchanges, and multilingual communication scenarios.
                        </span>
                    </h3>
                    <Space size={"large"} className={"desktop-welcome-content-button"}>
                        <Button size={"large"}>START</Button>
                        <Button size={"large"}>CLOSE</Button>
                    </Space>
                </Space>
            </div>
        </div>
    )
}