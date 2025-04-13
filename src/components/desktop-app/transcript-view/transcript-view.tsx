import "./transcript-view.scss";
import React from "react";
import {Flex, Space} from "antd";
import {Welcome as TextBlockView} from "@ant-design/x";
import {SpeakerIcon01} from "@src/icons/speaker/speaker-icon-01.tsx";
import {LikeIcon01} from "@src/icons/like/like-icon-01.tsx";
import {MoreIcon01} from "@src/icons/more/more-icon-01.tsx";

interface TranscriptViewProps {
    datetime: string;
    transcript: string;
}

export const TranscriptView: React.FC<TranscriptViewProps> = ({datetime, transcript}) => {
    return (
        <div className={"transcript-view"}>
            <TextBlockView className={"transcript-view-block"}
                           title={<div className={"transcript-view-title"}>
                               <Flex justify={"space-between"}>
                                   <Space align={"start"}>
                                       <span>{datetime}</span>
                                       <span>ROOM</span>
                                       <span>Speaker</span>
                                   </Space>
                                   <Space align={"end"} size={"middle"}>
                                       <div>
                                           <SpeakerIcon01 width={15} height={15} color={"#D0D4D9"}/>
                                       </div>
                                       <div>
                                           <LikeIcon01 width={15} height={15} color={"#D0D4D9"}/>
                                       </div>
                                       <div>
                                           <MoreIcon01 width={15} height={15} color={"#D0D4D9"}/>
                                       </div>
                                   </Space>
                               </Flex>
                           </div>}
                           description={<div className={"transcript-view-body"}>
                               <Space direction={"vertical"} size={"small"} style={{display: "flex"}}>
                                   <div>{transcript}</div>
                               </Space>
                           </div>}/>
        </div>
    )
}