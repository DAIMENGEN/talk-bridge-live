import "./app-about-float-button.scss";
import {FloatButton} from "antd";
import {AboutIcon} from "@src/icons";
import {DARK_MODE_BACKGROUND_COLOR} from "@src/theme/theme.ts";

export const AppAboutFloatButton = () => {
  return (
      <FloatButton icon={<AboutIcon width={20} height={20} color={DARK_MODE_BACKGROUND_COLOR}/>}
                   tooltip={"About"}
      />
  )
}