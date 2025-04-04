import "./App.scss";
import {BrowserRouter} from "react-router-dom";
import {TalkBridgeLiveRoute} from "@src/route/talk-bridge-live-route.tsx";

function App() {
    return (
        <BrowserRouter>
            <TalkBridgeLiveRoute/>
        </BrowserRouter>
    );
}

export default App;
