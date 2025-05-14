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
    SilenceStreakCountAdjust
} from "@src/components/desktop-app/app-settings/silence-streak-count-adjust/silence-streak-count-adjust.tsx";
import {
    AudioGapThresholdAdjust
} from "@src/components/desktop-app/app-settings/audio-gap-threshold-adjust/audio-gap-threshold-adjust.tsx";
import {DARK_MODE_FONT_COLOR} from "@src/theme/theme.ts";

export const AppSettings = () => {
    const appDispatch = useAppDispatch();
    const isOpenSettings = useAppSelector((state) => state.appSettings.isOpenSettings);
    return (
        <Drawer title={"App settings"}
                open={isOpenSettings}
                className={"app-settings"}
                width={"80%"}
                destroyOnClose={true}
                closeIcon={<CloseOutlined style={{fontSize: 20, color: DARK_MODE_FONT_COLOR}}/>}
                onClose={() => appDispatch(setIsOpenSettings(false))}>
            <Space direction="vertical" size="middle" style={{display: 'flex'}}>
                <Card title="Microphone Settings" variant="borderless" hoverable>
                    <LabelBlock label="Microphone Select" content={<MicrophoneSelect/>}/>
                    <LabelBlock label="Microphone Gain Adjust" content={<MicrophoneGainAdjust/>}/>
                    <LabelBlock label="Speech Threshold Adjust" content={<SpeechThresholdAdjust/>}/>
                    <LabelBlock label="Silence Streak Count Adjust" content={<SilenceStreakCountAdjust/>}/>
                    <LabelBlock label="Audio Gap Threshold Adjust" content={<AudioGapThresholdAdjust/>}/>
                    <LabelBlock label="Human Voice Detection" content={<MicrophoneHumanVoiceDetection/>}/>
                </Card>
            </Space>
        </Drawer>
    )
}