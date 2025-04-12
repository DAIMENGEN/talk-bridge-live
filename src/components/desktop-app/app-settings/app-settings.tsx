import "./app-settings.scss";
import {Card, Drawer, Space} from "antd";
import {CloseOutlined} from "@ant-design/icons";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {setIsOpenSettings} from "@src/store/features/app-settings-slice.ts";
import {MicrophoneSelect} from "@src/components/desktop-app/app-settings/microphone-select/microphone-select.tsx";
import {SpeakerSelect} from "@src/components/desktop-app/app-settings/speaker-select/speaker-select.tsx";
import {LabelBlock} from "@src/components/label-block/label-block.tsx";
import {
    MicrophoneGainControl
} from "@src/components/desktop-app/app-settings/microphone-gain-control/microphone-gain-control.tsx";
import {
    SpeakerVolumeControl
} from "@src/components/desktop-app/app-settings/speaker-volume-control/speaker-volume-control.tsx";
import {
    HumanVoiceDetection
} from "@src/components/desktop-app/app-settings/human-voice-detection/human-voice-detection.tsx";

export const AppSettings = () => {
    const appDispatch = useAppDispatch();
    const isOpenSettings = useAppSelector((state) => state.appSettings.isOpenSettings);
    return (
        <Drawer title={"App settings"}
                open={isOpenSettings}
                className={"app-settings"}
                width={"80%"}
                destroyOnClose={true}
                closeIcon={<CloseOutlined style={{fontSize: 20, color: "#D0D4D9"}}/>}
                onClose={() => appDispatch(setIsOpenSettings(false))}>
            <Space direction="vertical" size="middle" style={{display: 'flex'}}>
                <Card title="Microphone Settings" variant="borderless" hoverable>
                    <LabelBlock label="Select Microphone" content={<MicrophoneSelect/>}/>
                    <LabelBlock label="Gain Control" content={<MicrophoneGainControl/>}/>
                    <LabelBlock label="Human Voice Detection" content={<HumanVoiceDetection/>}/>
                </Card>

                <Card title="Speaker Settings" variant="borderless" hoverable>
                    <LabelBlock label="Select Speaker" content={<SpeakerSelect/>}/>
                    <LabelBlock label="Speaker Volume Control" content={<SpeakerVolumeControl/>}/>
                </Card>
            </Space>
        </Drawer>
    )
}