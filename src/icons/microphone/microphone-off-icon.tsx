import React from "react";
import Icon from "@ant-design/icons";
import {StyleUtils} from "@src/style-utils.ts";

export const MicrophoneOffIcon: React.FC<{ width: number, height: number, color: string }> = (props) => {

    const microphoneOffIcon = () => (
        <svg className="microphone-off-icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
             width={`${StyleUtils.numberToPixels(props.width)}`} height={`${StyleUtils.numberToPixels(props.height)}`}>
            <path
                d="M202.096 630.464l80.816-52.816A254.896 254.896 0 0 1 256 464a48 48 0 1 0-96 0l0.016 0.128-0.016 0.24c0 60.192 15.424 116.624 42.096 166.096zM704 302.32V192c0-106.032-85.968-192-192-192S320 85.968 320 192v256c0 32.752 8.24 63.568 22.704 90.544L704 302.32zM952 241.6a24 24 0 0 0-37.12-20.064v-0.016l-0.288 0.176L81.264 767.12a24 24 0 1 0 26.272 40.144v0.016l0.272-0.192L941.12 261.68v-0.016c6.56-4.272 10.88-11.664 10.88-20.064zM701.744 475.824l-240.4 157.2c16.16 4.432 33.088 6.976 50.656 6.976 96.56 0 176.256-71.36 189.744-164.176zM816 416a48 48 0 0 0-48 48c-0.064 129.472-96.288 236.144-221.056 253.264a274.944 274.944 0 0 1-32.944 2.624v0.032c-0.64 0-1.248 0.08-1.888 0.08-47.344 0-91.52-13.056-129.568-35.456l-86.896 56.816a350.4 350.4 0 0 0 168.336 71.04V928H304a48 48 0 1 0 0 96h416a48 48 0 1 0 0-96H560v-115.6c171.632-23.312 304-170.032 304-348.032l-0.016-0.24L864 464a48 48 0 0 0-48-48z"
                fill={props.color}></path>
        </svg>
    )

    return <Icon component={microphoneOffIcon} {...props}/>
}