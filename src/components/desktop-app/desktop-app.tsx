import "./desktop-app.scss";
import {FloatButton} from "antd";
import {Outlet} from "react-router-dom";
import {Toolbar} from "@src/components/desktop-app/toolbar/toolbar.tsx";
import {MoreIcon} from "@src/icons/more/more-icon.tsx";
import {AboutIcon} from "@src/icons/about/about-icon.tsx";
import {IssueIcon} from "@src/icons/issue/issue-icon.tsx";
import {ManualIcon} from "@src/icons/manual/manual-icon.tsx";
import {UpdateIcon} from "@src/icons/update/update-icon.tsx";

export const DesktopApp = () => {
    return (
        <div className={"desktop-app"}>
            <Toolbar/>
            <Outlet/>
            <FloatButton.Group trigger={"click"}
                               icon={<MoreIcon width={20} height={20} color={"#141414"}/>}>
                <FloatButton icon={<ManualIcon width={20} height={20} color={"#141414"}/>}
                             tooltip={"User manual"}
                />
                <FloatButton icon={<IssueIcon width={20} height={20} color={"#141414"}/>}
                             tooltip={"Report an issue"}
                />
                <FloatButton icon={<AboutIcon width={20} height={20} color={"#141414"}/>}
                             tooltip={"About"}
                />
                <FloatButton icon={<UpdateIcon width={20} height={20} color={"#141414"}/>}
                             tooltip={"Check for updates"}
                />
            </FloatButton.Group>
        </div>
    )
}