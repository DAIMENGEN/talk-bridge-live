import "./settings-float-button.scss";
import {AudioIcon, PrivacyIcon, SettingsIcon} from "@src/icons";
import {Button, FloatButton, Layout, Menu, Select, Slider, Space} from "antd";
import {useCallback, useEffect, useState} from "react";
import {DraggableModal, VoiceActivityIndicator} from "@src/components/common";
import {DARK_MODE_BACKGROUND_COLOR, DARK_MODE_FONT_COLOR} from "@src/theme/theme.ts";
import {MenuItem, SelectOption} from "@src/types.ts";
import {TauriDeviceService} from "@src/tauri-services/tauri-device-service.ts";
import {TauriService} from "@src/tauri-services/tauri-service.ts";
import {log} from "@src/logger.ts";
import {UnlistenFn} from "@tauri-apps/api/event";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {TauriStateService} from "@src/tauri-services/tauri-state-service.ts";
import {
    setMicrophoneGain,
    setMicrophoneName,
    setSilenceStreakThreshold,
    setSpeechThreshold
} from "@src/store/features/app-settings-slice.ts";

export const SettingsFloatButton = () => {
    const items: MenuItem[] = [
        {
            key: "1",
            icon: <SettingsIcon width={20} height={20} color={DARK_MODE_FONT_COLOR}/>,
            label: "General",
        },
        {
            key: "2",
            icon: <AudioIcon width={20} height={20} color={DARK_MODE_FONT_COLOR}/>,
            label: "Audio",
        },
        {
            key: "3",
            icon: <PrivacyIcon width={20} height={20} color={DARK_MODE_FONT_COLOR}/>,
            label: "Privacy",
        },
    ];
    const [openSettings, setOpenSettings] = useState(false);
    const [selectedKey, setSelectedKey] = useState<string>("1");
    const renderSettingsComponent = useCallback(() => {
        switch (selectedKey) {
            case "1":
                return <GeneralSettings/>;
            case "2":
                return <AudioSettings/>;
            case "3":
                return <PrivacySettings/>;
            default:
                return <GeneralSettings/>;
        }
    }, [selectedKey]);
    return (
        <>
            <FloatButton icon={<SettingsIcon width={20} height={20} color={DARK_MODE_BACKGROUND_COLOR}/>}
                         tooltip={"Settings"}
                         onClick={() => setOpenSettings(true)}
            />
            <DraggableModal closable
                            footer={null}
                            width={"85%"}
                            centered={true}
                            title={"Settings"}
                            open={openSettings}
                            destroyOnClose={true}
                            classNames={{
                                wrapper: "setting-draggable-modal",
                            }}
                            maskClosable={false}
                            onCancel={() => {
                                setSelectedKey("1");
                                setOpenSettings(false);
                            }}>
                <Layout>
                    <Layout.Sider>
                        <Menu defaultSelectedKeys={[selectedKey]} mode="inline" theme={"dark"} items={items}
                              onSelect={({key}) => setSelectedKey(key)}/>
                    </Layout.Sider>
                    <Layout>
                        {renderSettingsComponent()}
                    </Layout>
                </Layout>
            </DraggableModal>
        </>
    )
}

export const GeneralSettings = () => {
    return (
        <>
            <Layout.Header>
                General
            </Layout.Header>
            <Layout.Content>
                Content
            </Layout.Content>
        </>
    )
}

export const AudioSettings = () => {
    const appDispatch = useAppDispatch();
    const [speakerNames, setSpeakerNames] = useState<SelectOption[]>([]);
    const [microphoneNames, setMicrophoneNames] = useState<SelectOption[]>([]);
    const [isTestingMicrophone, setIsTestingMicrophone] = useState(false);
    const [microphoneSpeechProbability, setMicrophoneSpeechProbability] = useState<number>(0);
    const microphoneGain = useAppSelector((state) => state.appSettings.microphone.gain);
    const microphoneName = useAppSelector((state) => state.appSettings.microphone.name);
    const speechThreshold = useAppSelector((state) => state.appSettings.speechThreshold);
    const silenceStreakThreshold = useAppSelector((state) => state.appSettings.silenceStreakThreshold);

    useEffect(() => {
        TauriDeviceService.listSpeakerNames().then(n => setSpeakerNames(n.map((name) => ({
            value: name,
            label: name
        }))));
        TauriDeviceService.listMicrophoneNames().then(n => setMicrophoneNames(n.map((name) => ({
            value: name,
            label: name
        }))));
    }, []);

    useEffect(() => {
        let unlisten: UnlistenFn;
        let cancelled = false;
        if (isTestingMicrophone && microphoneName) {
            TauriDeviceService.testMicrophone(microphoneName).then(eventName => {
                if (cancelled) return;
                TauriService.listen<number>(eventName, (event) => {
                    setMicrophoneSpeechProbability(event.payload);
                }).then(result => {
                    if (!cancelled) {
                        unlisten = result;
                    }
                });
            });
        }
        return () => {
            cancelled = true;
            if (isTestingMicrophone) {
                unlisten && unlisten();
                TauriDeviceService.stopTestMicrophone().catch(log.error);
            }
        };
    }, [microphoneName, isTestingMicrophone]);

    return (
        <>
            <Layout.Header style={{lineHeight: "40px", fontSize: "25px"}}>
                Audio Settings
            </Layout.Header>
            <Layout.Content>
                <Space direction="vertical" size="large" style={{display: "flex"}}>
                    <Space direction="vertical" size="middle" style={{display: "flex"}}>
                        <div className={"speaker-text"}>
                            Speaker
                        </div>
                        <Space size={"small"} rootClassName={"select-container"}>
                            <Select options={speakerNames} rootClassName={"speaker-select"}
                                    popupClassName={"speaker-select-options"}/>
                            <Button>Test</Button>
                        </Space>
                        <div className={"indicator-container"}>
                            <div>Output Volume</div>
                            <div style={{width: "100%"}}><VoiceActivityIndicator probability={0}/></div>
                        </div>
                        <div className={"slider-container"}>
                            <div>Volume</div>
                            <Slider styles={{
                                root: {
                                    width: "100%"
                                }
                            }}/>
                        </div>
                    </Space>
                    <Space direction="vertical" size="middle" style={{display: "flex"}}>
                        <div className={"microphone-text"}>
                            Microphone
                        </div>
                        <Space size={"small"} rootClassName={"select-container"}>
                            <Select options={microphoneNames}
                                    defaultValue={microphoneName}
                                    rootClassName={"microphone-select"}
                                    placeholder={"Please select a microphone"}
                                    popupClassName={"microphone-select-options"}
                                    onChange={(value: string) => {
                                        setIsTestingMicrophone(false);
                                        appDispatch(setMicrophoneName(value));
                                    }}/>
                            <Button
                                onClick={() => {
                                    isTestingMicrophone && setMicrophoneSpeechProbability(0);
                                    setIsTestingMicrophone(value => !value);
                                }}>{isTestingMicrophone ? "Stop" : "Test"}</Button>
                        </Space>
                        <div className={"indicator-container"}>
                            <div>Speech Detection</div>
                            <div style={{width: "100%"}}><VoiceActivityIndicator probability={microphoneSpeechProbability}/></div>
                        </div>
                        <div className={"slider-container"}>
                            <div>Gain</div>
                            <Slider min={0} max={3} step={0.1} defaultValue={microphoneGain}
                                    onChangeComplete={(value) => {
                                        TauriStateService.setMicrophoneGain(value).then(_ => {
                                            appDispatch(setMicrophoneGain(value));
                                        }).catch(log.error);
                                    }} styles={{
                                root: {
                                    width: "100%"
                                }
                            }}/>
                        </div>
                        <div className={"slider-container"}>
                            <div>Speech Threshold</div>
                            <Slider min={0} max={1} step={0.05} defaultValue={speechThreshold}
                                    onChangeComplete={(value) => {
                                        TauriStateService.setSpeechThreshold(value).then(_ => {
                                            appDispatch(setSpeechThreshold(value));
                                        }).catch(log.error);
                                    }} styles={{
                                root: {
                                    width: "100%"
                                }
                            }}/>
                        </div>
                        <div className={"slider-container"}>
                            <div>Silence Streak Threshold</div>
                            <Slider min={0} max={10} step={1} defaultValue={silenceStreakThreshold}
                                    onChangeComplete={(value) => {
                                        TauriStateService.setSilenceStreakThreshold(value).then(_ => {
                                            appDispatch(setSilenceStreakThreshold(value));
                                        }).catch(log.error);
                                    }} styles={{
                                root: {
                                    width: "100%"
                                }
                            }}/>
                        </div>
                    </Space>
                </Space>
            </Layout.Content>
        </>
    )
}

export const PrivacySettings = () => {
    return (
        <>
            <Layout.Header>
                Privacy
            </Layout.Header>
            <Layout.Content>
                Content
            </Layout.Content>
        </>
    )
}