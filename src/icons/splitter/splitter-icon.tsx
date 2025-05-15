import React from "react";
import Icon from "@ant-design/icons";
import {StyleUtils} from "@src/style-utils.ts";

export const SplitterIcon: React.FC<{ width: number, height: number, color: string }> = (props) => {
    const splitterIcon = () => (
        <svg className="splitter-icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
             width={`${StyleUtils.numberToPixels(props.width)}`} height={`${StyleUtils.numberToPixels(props.height)}`}>
            <path
                d="M512 64a32 32 0 0 1 32 32v832a32 32 0 1 1-64 0v-832A32 32 0 0 1 512 64zM288 128a32 32 0 0 1 0 64H128v640h160a32 32 0 1 1 0 64H128a64 64 0 0 1-64-64V192a64 64 0 0 1 64-64h160zM896 128a64 64 0 0 1 64 64v640a64 64 0 0 1-64 64h-160a32 32 0 1 1 0-64H896V192h-160a32 32 0 1 1 0-64H896z"
                fill={props.color}></path>
        </svg>
    )
    return <Icon component={splitterIcon} {...props}/>
}