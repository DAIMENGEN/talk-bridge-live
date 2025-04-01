import "./App.scss";
import {Button, Flex} from "antd";
// import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {UpdateBadgeIcon} from "@src/icons/update/update-badge-icon.tsx";
import {UpdateDoneIcon} from "@src/icons/update/update-done-icon.tsx";
import {CloseOutlined, FullscreenExitOutlined, FullscreenOutlined, PushpinOutlined} from "@ant-design/icons";

function App() {

    return (
        <div className={"container"}>
            <Flex gap={0} justify={"space-between"} className={"toolbar"}>
                <div className={"toolbar-left"}>
                    <Button>Icon</Button>
                </div>
                <div className={"toolbar-center"} onMouseDown={() => getCurrentWindow().startDragging()}/>
                <div className={"toolbar-right"}>
                    <Button type={"text"} icon={<UpdateBadgeIcon width={20} height={20} color={"#bfbfbf"}/>}/>
                    <Button type={"text"} icon={<UpdateDoneIcon width={20} height={20} color={"#bfbfbf"}/>}/>
                    <Button type={"text"} icon={<PushpinOutlined style={{fontSize: 20, color: "#bfbfbf"}}/>}/>
                    <Button type={"text"} icon={<FullscreenOutlined style={{fontSize: 20, color: "#bfbfbf"}}/>}/>
                    <Button type={"text"} icon={<FullscreenExitOutlined style={{fontSize: 20, color: "#bfbfbf"}}/>}/>
                    <Button type={"text"} icon={<CloseOutlined style={{fontSize: 20, color: "#bfbfbf"}}/>}
                            onClick={() => getCurrentWindow().close()}/>
                </div>
            </Flex>
            <h1>Welcome to talk bridge live!</h1>
        </div>
    );
}

export default App;
