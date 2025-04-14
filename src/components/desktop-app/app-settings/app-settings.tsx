import "./app-settings.scss";
import {Card, Drawer, Space} from "antd";
import {CloseOutlined} from "@ant-design/icons";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {setIsOpenSettings} from "@src/store/features/app-settings-slice.ts";
import {MicrophoneSelect} from "@src/components/desktop-app/app-settings/microphone-select/microphone-select.tsx";
import {LabelBlock} from "@src/components/label-block/label-block.tsx";
import {
    MicrophoneGainAdjust
} from "@src/components/desktop-app/app-settings/microphone-gain-adjust/microphone-gain-adjust.tsx";
import {
    MicrophoneHumanVoiceDetection
} from "@src/components/desktop-app/app-settings/microphone-human-voice-detection/microphone-human-voice-detection.tsx";
import {
    SpeechThresholdAdjust
} from "@src/components/desktop-app/app-settings/speech-threshold-adjust/speech-threshold-adjust.tsx";
import {
    AudioToleranceAdjust
} from "@src/components/desktop-app/app-settings/audio-tolerance-adjust/audio-tolerance-adjust.tsx";

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
                    <LabelBlock label="Microphone Select" content={<MicrophoneSelect/>}/>
                    <LabelBlock label="Microphone Gain Adjust" content={<MicrophoneGainAdjust/>}/>
                    <LabelBlock label="Audio Tolerance Adjust" content={<AudioToleranceAdjust/>}/>
                    <LabelBlock label="Speech Threshold Adjust" content={<SpeechThresholdAdjust/>}/>
                    <LabelBlock label="Human Voice Detection" content={<MicrophoneHumanVoiceDetection/>}/>
                </Card>
            </Space>
        </Drawer>
    )
}