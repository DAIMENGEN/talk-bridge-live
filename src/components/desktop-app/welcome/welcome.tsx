import "./welcome.scss";
import {useState} from "react";
import {Button, Space} from "antd";
import {TauriDeviceService} from "@src/tauri-services/tauri-device-service.ts";
import {TauriAudioRecorderService} from "@src/tauri-services/tauri-audio-recorder-service.ts";

export const Welcome = () => {
    const [deviceName, setDeviceName] = useState<string>();

    return (
        <div className={"desktop-welcome"}>
            <div className={"desktop-welcome-content"}>
                <Space direction="vertical" size="small">
                    <h1>Talk Bridge Live</h1>
                    <h2>跨越语言障碍的 AI 同声翻译软件</h2>
                    <h3>
                        Talk Bridge Live 是一款由 Advantest 精心打造的 AI 同声翻译软件，专为企业会议、国际交流及多语言沟通场景设计。
                        它依托先进的人工智能与实时语音识别技术，能够精准、流畅地将演讲者的语音即时翻译成目标语言，并以字幕或语音的方式呈现，让不同语言的与会者无缝沟通，仿佛置身同一间会议室。
                        Talk Bridge Live 让语言不再成为沟通的障碍，让企业会议更加高效、顺畅。无论是跨国团队协作、行业论坛，还是国际商务洽谈，都能助您轻松实现零距离交流！
                    </h3>
                    <div className={"desktop-welcome-content-button"}>
                        <Button size={"large"} onClick={() => {
                            if (deviceName) {
                                TauriAudioRecorderService.startRecording(deviceName).then(() => {
                                    console.log("start recording")
                                });
                            }
                        }}>开始录音</Button>
                        <Button size={"large"} onClick={() => {
                            TauriAudioRecorderService.stopRecording().then(() => {
                                console.log("stop recording")
                            });
                        }}>停止录音</Button>

                        <Button size={"large"} onClick={() => {
                            TauriDeviceService.listMicrophoneNames().then((device_names) => {
                                console.log("Device names: ", device_names);
                                setDeviceName(device_names[0]);
                            });
                        }}>设备列表</Button>
                    </div>
                </Space>
            </div>
        </div>
    )
}