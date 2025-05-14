import React from "react";
import Icon from "@ant-design/icons";
import {StyleUtils} from "@src/style-utils.ts";

export const MicrophoneOnIcon: React.FC<{ width: number, height: number, color: string }> = (props) => {
    const microphoneOnIcon = () => (
        <svg className="microphone-icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
             width={`${StyleUtils.numberToPixels(props.width)}`} height={`${StyleUtils.numberToPixels(props.height)}`}>
            <path
                d="M512 640c106.032 0 192-85.968 192-192V192c0-106.032-85.968-192-192-192S320 85.968 320 192v256c0 106.048 85.968 192 192 192z m352-175.632l-0.016-0.24L864 464a48 48 0 1 0-96 0c-0.064 141.36-114.64 255.936-256 256-141.36-0.064-255.936-114.64-256-256a48 48 0 1 0-96 0l0.016 0.128-0.016 0.24c0 178 132.384 324.72 304 348.032V928H304a48 48 0 1 0 0 96h416a48 48 0 1 0 0-96H560v-115.6c171.632-23.312 304-170.032 304-348.032z"
                fill={props.color}></path>
        </svg>
    )
    return <Icon component={microphoneOnIcon} {...props}/>
}