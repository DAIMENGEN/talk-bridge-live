import "./desktop-app.scss";
import {useEffect, useRef} from "react";
import {Outlet} from "react-router-dom";
import {Toolbar} from "@src/components/desktop-app/toolbar/toolbar.tsx";
import {FloatButton} from "antd";
import {MoreIcon01} from "@src/icons/more/more-icon-01.tsx";
import {AboutIcon01} from "@src/icons/about/about-icon-01.tsx";
import {IssueIcon01} from "@src/icons/issue/issue-icon-01.tsx";
import {ManualIcon01} from "@src/icons/manual/manual-icon-01.tsx";
import {UpdateDoneIcon} from "@src/icons/update/update-done-icon.tsx";

export const DesktopApp = () => {
    const toolbarWrapperRef = useRef<HTMLDivElement>(null);
    useEffect(() => {
        const handleMouseEnter = () => {
            if (toolbarWrapperRef.current) {
                toolbarWrapperRef.current.style.display = 'block';
            }
        };
        const handleMouseLeave = () => {
            if (toolbarWrapperRef.current) {
                toolbarWrapperRef.current.style.display = 'none';
            }
        };
        document.addEventListener('mouseenter', handleMouseEnter);
        document.addEventListener('mouseleave', handleMouseLeave);
        return () => {
            document.removeEventListener('mouseenter', handleMouseEnter);
            document.removeEventListener('mouseleave', handleMouseLeave);
        };
    }, []);

    return (
        <div className={"desktop-app"}>
            <Toolbar/>
            <Outlet/>
            <FloatButton.Group trigger={"click"}
                               icon={<MoreIcon01 width={20} height={20} color={"#141414"}/>}>
                <FloatButton icon={<ManualIcon01 width={20} height={20} color={"#141414"}/>}
                             tooltip={"User manual"}
                />
                <FloatButton icon={<IssueIcon01 width={20} height={20} color={"#141414"}/>}
                             tooltip={"Report an issue"}
                />
                <FloatButton icon={<AboutIcon01 width={20} height={20} color={"#141414"}/>}
                             tooltip={"About"}
                />
                <FloatButton icon={<UpdateDoneIcon width={20} height={20} color={"#141414"}/>}
                             tooltip={"Check for updates"}
                />
            </FloatButton.Group>
        </div>
    )
}