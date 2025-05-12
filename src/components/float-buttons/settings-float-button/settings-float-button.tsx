import "./settings-float-button.scss";
import {SettingsIcon} from "@src/icons";
import {FloatButton} from "antd";

export const SettingsFloatButton = () => {
    return (
        <FloatButton icon={<SettingsIcon width={20} height={20} color={"#141414"}/>}
                     tooltip={"Settings"}
        />
    )
}