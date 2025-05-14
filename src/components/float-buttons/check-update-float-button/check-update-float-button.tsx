import "./check-update-float-button.scss";
import {FloatButton} from "antd";
import {UpdateIcon} from "@src/icons";
import {DARK_MODE_BACKGROUND_COLOR} from "@src/theme/theme.ts";

export const CheckUpdateFloatButton = () => {
  return (
      <FloatButton icon={<UpdateIcon width={20} height={20} color={DARK_MODE_BACKGROUND_COLOR}/>}
                   tooltip={"Check for updates"}
      />
  )
}