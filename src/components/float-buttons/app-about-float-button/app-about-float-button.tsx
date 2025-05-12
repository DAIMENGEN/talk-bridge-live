import "./app-about-float-button.scss";
import {FloatButton} from "antd";
import {AboutIcon} from "@src/icons";

export const AppAboutFloatButton = () => {
  return (
      <FloatButton icon={<AboutIcon width={20} height={20} color={"#141414"}/>}
                   tooltip={"About"}
      />
  )
}