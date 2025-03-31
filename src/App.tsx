import "./App.scss";
import {Button, Flex} from "antd";
// import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {CloseIcon01} from "@src/icons/close/close-icon-01.tsx";
import {FullScreen01} from "@src/icons/screen/full-screen-01.tsx";
import {UnFullScreen01} from "@src/icons/screen/un-full-screen-01.tsx";

function App() {

    return (
        <div className={"container"}>
            <Flex gap={0} justify={"space-between"} className={"toolbar"}>
                <div className={"toolbar-left"}>
                    <Button>Icon</Button>
                </div>
                <div className={"toolbar-center"} onMouseDown={() => getCurrentWindow().startDragging()}/>
                <div className={"toolbar-right"}>
                    <Button type={"text"} icon={<FullScreen01 width={20} height={20} color={"#bfbfbf"}/>}/>
                    <Button type={"text"} icon={<UnFullScreen01 width={20} height={20} color={"#bfbfbf"}/>}/>
                    <Button type={"text"} icon={<CloseIcon01 width={20} height={20} color={"#bfbfbf"}/>}
                            onClick={() => getCurrentWindow().close()}/>
                </div>
            </Flex>
            <h1>Welcome to talk bridge live!</h1>
        </div>
    );
}

export default App;
