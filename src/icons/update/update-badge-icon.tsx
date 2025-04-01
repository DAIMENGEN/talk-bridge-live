import React from "react";
import Icon from "@ant-design/icons";
import {StyleUtils} from "@src/style-utils";

export const UpdateBadgeIcon: React.FC<{ width: number, height: number, color: string }> = (props) => {
    const icon = () => (
        <svg className="update-badge-icon-01" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
             width={`${StyleUtils.numberToPixels(props.width)}`} height={`${StyleUtils.numberToPixels(props.height)}`}>
            <path d="M336 792h-40a240 240 0 0 1 0-480h56a32 32 0 0 1 0 64h-56a176 176 0 0 0 0 352h40a32 32 0 0 1 0 64z"
                  fill={props.color}></path>
            <path
                d="M736 428a32 32 0 0 1-32-32v-40a192 192 0 0 0-382.08-27.44 32 32 0 1 1-63.36-9.04A256 256 0 0 1 768 356v40a32 32 0 0 1-32 32z"
                fill={props.color}></path>
            <path
                d="M728 792H496a32 32 0 0 1 0-64h232a177.52 177.52 0 0 0 6.8-354.88A32.48 32.48 0 0 1 704 339.44a32 32 0 0 1 33.2-31.04A241.92 241.92 0 0 1 728 792z"
                fill={props.color}></path>
            <path
                d="M592 924a32 32 0 0 1-22.64-9.36L444.96 790.16a48 48 0 0 1 0-67.84l124.4-124.48a32 32 0 0 1 45.28 45.28L501.52 756.24l113.12 113.12A32 32 0 0 1 592 924z"
                fill={props.color}></path>
            <circle cx="800.0" cy="700.0" r="150" fill="#e81123"/>
        </svg>
    )
    return <Icon component={icon} {...props}/>
}