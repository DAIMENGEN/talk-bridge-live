import "./utterance-content.scss";
import React from "react";
import {Flex, Space} from "antd";
import {LikeIcon, MoreIcon, SpeakerIcon} from "@src/icons";
import {DARK_MODE_FONT_COLOR} from "@src/theme/theme.ts";
import {Utterance} from "@src/store/features/meeting-content-slice.ts";

export const UtteranceContent: React.FC<Utterance> = (props) => {
    return (
        <Space rootClassName={"utterance-content"} direction={"vertical"} size={"small"}>
            <Flex justify={"space-between"}>
                <Space align={"center"} size={"small"} style={{fontSize: 16}}>
                    <span>{props.datetime}</span>
                    <span>{props.speaker}</span>
                </Space>
                <Space align={"center"} size={"large"}>
                    <div>
                        <SpeakerIcon width={16} height={16} color={DARK_MODE_FONT_COLOR}/>
                    </div>
                    <div>
                        <LikeIcon width={16} height={16} color={DARK_MODE_FONT_COLOR}/>
                    </div>
                    <div>
                        <MoreIcon width={16} height={16} color={DARK_MODE_FONT_COLOR}/>
                    </div>
                </Space>
            </Flex>
            <Space direction={"vertical"} size={"small"} style={{display: "flex", fontSize: 18}}>
                <span>{props.speechText}</span>
            </Space>
        </Space>
    )
}