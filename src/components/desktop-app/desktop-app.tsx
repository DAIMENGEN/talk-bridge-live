import "./desktop-app.scss";
import {Outlet} from "react-router-dom";
import {Toolbar} from "@src/components/desktop-app/toolbar/toolbar.tsx";

export const DesktopApp = () => {
    return (
        <div className={"desktop-app"}>
            <Toolbar/>
            <Outlet/>
        </div>
    )
}