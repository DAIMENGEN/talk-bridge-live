import "./user-manual-float-button.scss";
import {FloatButton} from "antd";
import {ManualIcon} from "@src/icons";
export const UserManualFloatButton = () => {
    return (
        <FloatButton icon={<ManualIcon width={20} height={20} color={"#141414"}/>}
                     tooltip={"User manual"}
        />
    )
}