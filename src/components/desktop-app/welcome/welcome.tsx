import "./welcome.scss";
import {Button, Space} from "antd";
import {invoke} from "@tauri-apps/api/core";
import {Toolbar} from "@src/components/desktop-app/toolbar/toolbar.tsx";

export const Welcome = () => {

    return (
        <div className={"desktop-welcome"}>
            <div className={"frosted-glass"}/>
            <div className={"desktop-welcome-body"}>
                <Toolbar/>
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
                                invoke("start_recording").then(() => {
                                    console.log("start recording")
                                });
                            }}>开始录音</Button>
                            <Button size={"large"} onClick={() => {
                                invoke("stop_recording").then(() => {
                                    console.log("stop recording")
                                });
                            }}>停止录音</Button>
                        </div>
                    </Space>
                </div>
            </div>
        </div>
    )
}