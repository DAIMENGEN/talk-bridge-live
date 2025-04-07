import {Route, Routes} from "react-router-dom";
import {DesktopApp} from "@src/components/desktop-app/desktop-app.tsx";
import {Welcome} from "@src/components/desktop-app/welcome/welcome.tsx";
import {AppHome} from "@src/components/desktop-app/app-home/app-home.tsx";

export const TalkBridgeLiveRoute = () => {
    return (
        <Routes>
            <Route path={"/"} element={<DesktopApp/>}>
                <Route index element={<Welcome/>}/>
                <Route path={"/app-home"} element={<AppHome/>}/>
            </Route>
        </Routes>
    )
}