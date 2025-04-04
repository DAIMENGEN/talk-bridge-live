import "./App.scss";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {DesktopApp} from "@src/components/desktop-app/desktop-app.tsx";

function App() {

    const [greetMsg, setGreetMsg] = useState("");
    // const [name, setName] = useState("");

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        setGreetMsg(await invoke("greet", {name: "dai"}));
        await invoke("start_recording");
        console.log(greetMsg)

        setTimeout(async () => {
            await invoke("stop_recording");
        }, 5000);
    }

    useEffect(() => {
        greet().then();
    });

    return (
        <DesktopApp/>
    );
}

export default App;
