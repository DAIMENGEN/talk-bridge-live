import "./desktop-app.scss";
import {Outlet} from "react-router-dom";
import {Toolbar} from "@src/components/desktop-app/toolbar/toolbar.tsx";
import {useEffect, useRef} from "react";

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
            <div className={"frosted-glass"}/>
            <div className={"toolbar-wrapper"} ref={toolbarWrapperRef}>
                <Toolbar/>
            </div>
            <Outlet/>
        </div>
    )
}