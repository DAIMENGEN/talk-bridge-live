import "./app-home.scss";
import {AppSettings} from "@src/components/desktop-app/app-settings/app-settings.tsx";

export const AppHome = () => {
    return (
        <div className={"app-home"}>
            <AppSettings/>
        </div>
    )
}