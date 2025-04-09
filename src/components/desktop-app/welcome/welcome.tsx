import "./welcome.scss";
import {Button, Space} from "antd";
import {useNavigate} from "react-router-dom";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {useAppDispatch} from "@src/store/store.ts";
import {setIsShowSettingsButton} from "@src/store/features/app-settings-slice.ts";

export const Welcome = () => {
    const navigate = useNavigate();
    const appDispatch = useAppDispatch();
    return (
        <div className={"desktop-welcome"}>
            <Space direction="vertical" size="small">
                <h1>Talk Bridge Live</h1>
                <h2>AI Simultaneous Interpretation – Breaking Language Barriers</h2>
                <h3>
                        <span>
                            Talk Bridge Live is an AI simultaneous translation software meticulously developed by Advantest China R&D, designed for corporate meetings, international exchanges, and multilingual communication scenarios.
                        </span>
                </h3>
                <Space size={"large"} className={"desktop-welcome-content-button"}>
                    <Button size={"large"} onClick={() => {
                        navigate("/app-home");
                        appDispatch(setIsShowSettingsButton(true));
                    }}>START</Button>
                    <Button size={"large"} onClick={async () => await getCurrentWindow().close()}>CLOSE</Button>
                </Space>
            </Space>
        </div>
    )
}