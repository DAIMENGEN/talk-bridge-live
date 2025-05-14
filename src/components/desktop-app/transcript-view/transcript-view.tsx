import "./transcript-view.scss";
import React from "react";
import {Flex, Space} from "antd";
import {Welcome as TextBlockView} from "@ant-design/x";
import {SpeakerIcon} from "@src/icons/speaker/speaker-icon.tsx";
import {LikeIcon} from "@src/icons/like/like-icon.tsx";
import {MoreIcon} from "@src/icons/more/more-icon.tsx";
import {DARK_MODE_FONT_COLOR} from "@src/theme/theme.ts";

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
                                           <SpeakerIcon width={15} height={15} color={DARK_MODE_FONT_COLOR}/>
                                       </div>
                                       <div>
                                           <LikeIcon width={15} height={15} color={DARK_MODE_FONT_COLOR}/>
                                       </div>
                                       <div>
                                           <MoreIcon width={15} height={15} color={DARK_MODE_FONT_COLOR}/>
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