import "./user-manual-float-button.scss";
import {FloatButton} from "antd";
import {ManualIcon} from "@src/icons";
import {DARK_MODE_BACKGROUND_COLOR} from "@src/theme/theme.ts";
export const UserManualFloatButton = () => {
    return (
        <FloatButton icon={<ManualIcon width={20} height={20} color={DARK_MODE_BACKGROUND_COLOR}/>}
                     tooltip={"User manual"}
        />
    )
}