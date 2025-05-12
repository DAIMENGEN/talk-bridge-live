import "./toolbar.scss";
import {Button, Flex} from "antd";
import advantest from "@src/assets/svg/advantest.svg";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {CloseOutlined, FullscreenExitOutlined, FullscreenOutlined, PushpinOutlined} from "@ant-design/icons";
import {SettingsIcon} from "@src/icons/settings/settings-icon.tsx";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {setIsOpenSettings} from "@src/store/features/app-settings-slice.ts";
import ConditionalRenderer from "@src/components/conditional-renderer/conditional-renderer.tsx";
import {useState} from "react";

export const Toolbar = () => {
    const [isFullscreen, setIsFullscreen] = useState(false);
    const [isAlwaysOnTop, setIsAlwaysOnTop] = useState(false);
    const appDispatch = useAppDispatch();

    const {isShowSettingsButton} = useAppSelector(state => state.appSettings);

    return (
        <Flex gap={0} justify={"space-between"} className={"toolbar"}>
            <div className={"toolbar-left"}>
                <img src={advantest} alt="Advantest" style={{height: 15}}/>
            </div>
            <div className={"toolbar-center"} onMouseDown={() => getCurrentWindow().startDragging()}/>
            <div className={"toolbar-right"}>
                <ConditionalRenderer isVisible={isShowSettingsButton}
                                     childrenComponent={<Button type={"text"} onClick={() => {
                                         appDispatch(setIsOpenSettings(true));
                                     }} icon={<SettingsIcon width={20} height={20} color={"#D0D4D9"}/>}/>}/>
                <Button type={"text"}
                        icon={<PushpinOutlined style={{fontSize: 20, color: isAlwaysOnTop ? "#91003c" : "#D0D4D9"}}/>}
                        onClick={() => {
                            getCurrentWindow().setAlwaysOnTop(!isAlwaysOnTop).then(_ => {
                                setIsAlwaysOnTop(!isAlwaysOnTop);
                            });
                        }}
                />
                <Button type={"text"}
                        icon={isFullscreen ? <FullscreenExitOutlined style={{fontSize: 20, color: "#D0D4D9"}}/> :
                            <FullscreenOutlined style={{fontSize: 20, color: "#D0D4D9"}}/>}
                        onClick={() => {
                            getCurrentWindow().setFullscreen(!isFullscreen).then(_ => {
                                setIsFullscreen(!isFullscreen);
                            });
                        }}
                />
                <Button type={"text"} icon={<CloseOutlined style={{fontSize: 20, color: "#D0D4D9"}}/>}
                        onClick={() => getCurrentWindow().close()}/>
            </div>
        </Flex>
    )
}