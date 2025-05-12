import "./check-update-float-button.scss";
import {FloatButton} from "antd";
import {UpdateIcon} from "@src/icons";

export const CheckUpdateFloatButton = () => {
  return (
      <FloatButton icon={<UpdateIcon width={20} height={20} color={"#141414"}/>}
                   tooltip={"Check for updates"}
      />
  )
}