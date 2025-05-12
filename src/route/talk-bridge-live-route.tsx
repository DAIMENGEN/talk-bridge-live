import {Route, Routes} from "react-router-dom";
import {DesktopApp} from "@src/components/desktop-app/desktop-app.tsx";
import {Welcome} from "@src/components/desktop-app/welcome/welcome.tsx";
import {AppHome} from "@src/components/desktop-app/app-home/app-home.tsx";
import {JoinMeeting} from "@src/components/desktop-app/join-meeting/join-meeting.tsx";
import {MeetingRoom} from "@src/components/desktop-app/meeting-room/meeting-room.tsx";

export const TalkBridgeLiveRoute = () => {
    return (
        <Routes>
            <Route path={"/"} element={<DesktopApp/>}>
                <Route index element={<Welcome/>}/>
                <Route path={"/welcome"} element={<Welcome/>}/>
                <Route path={"/join-meeting"} element={<JoinMeeting/>}/>
                <Route path={"/meeting-room/:meetingId"} element={<MeetingRoom/>}/>
                <Route path={"/app-home"} element={<AppHome/>}/>
            </Route>
        </Routes>
    )
}