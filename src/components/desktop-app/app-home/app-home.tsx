import "./app-home.scss";
import {useEffect, useRef, useState} from "react";
import {useAppSelector} from "@src/store/store.ts";
import {UnlistenFn} from "@tauri-apps/api/event";
import {TauriAudioService} from "@src/tauri-services/tauri-audio-service.ts";
import {log} from "@src/logger.ts";
import {TauriService} from "@src/tauri-services/tauri-service.ts";
import {Flex} from "antd";
import {TranscriptView} from "@src/components/desktop-app/transcript-view/transcript-view.tsx";

interface TranscriptData {
    datetime: string;
    transcript: string;
}

export const AppHome = () => {
    const appHomeRef = useRef<HTMLDivElement>(null);
    const [transcripts, setTranscripts] = useState<TranscriptData[]>([]);
    const microphoneName = useAppSelector((state) => state.appSettings.microphoneName);
    useEffect(() => {
        const appHome = appHomeRef.current;
        if (appHome) {
            const frameId = requestAnimationFrame(() => {
                appHome.scrollTop = appHome.scrollHeight;
            });
            return () => cancelAnimationFrame(frameId);
        }
    }, [transcripts]);
    useEffect(() => {
        let unlisten: UnlistenFn;
        if (microphoneName) {
            TauriAudioService.startASR(microphoneName).then(eventName => {
                TauriService.listen<TranscriptData>(eventName, (event) => {
                    setTranscripts((prevTranscripts) => {
                        const newTranscripts = [...prevTranscripts, event.payload];
                        // 保证 transcripts 数组的长度不超过 100
                        return newTranscripts.length > 100 ? newTranscripts.slice(-100) : newTranscripts;
                    });
                }).then(result => {
                    unlisten = result;
                }).catch(error => {
                    log.error("Listen recording event error:", error)
                });
            });
        }
        return () => {
            unlisten && unlisten();
            TauriAudioService.stopASR().then(result => log.debug("Stop recording: ", result));
        }
    }, [microphoneName]);
    return (
        <div className={"app-home"} ref={appHomeRef}>
            <Flex vertical={true} gap={"small"}>
                {
                    transcripts.map((transcriptData, index) => {
                        return <TranscriptView key={index} datetime={transcriptData.datetime}
                                               transcript={transcriptData.transcript}/>
                    })
                }
            </Flex>
        </div>
    )
}