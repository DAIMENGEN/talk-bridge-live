import "./app-settings.scss";
import {Card, Drawer, Space} from "antd";
import {CloseOutlined} from "@ant-design/icons";
import {useAppDispatch, useAppSelector} from "@src/store/store.ts";
import {setIsOpenSettings} from "@src/store/features/app-settings-slice.ts";
import {LabelBlock} from "@src/components/label-block/label-block.tsx";
import {
    SpeechThresholdAdjust
} from "@src/components/desktop-app/app-settings/speech-threshold-adjust/speech-threshold-adjust.tsx";
import {
    SilenceStreakCountAdjust
} from "@src/components/desktop-app/app-settings/silence-streak-count-adjust/silence-streak-count-adjust.tsx";
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
                    <LabelBlock label="Speech Threshold Adjust" content={<SpeechThresholdAdjust/>}/>
                    <LabelBlock label="Silence Streak Count Adjust" content={<SilenceStreakCountAdjust/>}/>
                </Card>
            </Space>
        </Drawer>
    )
}