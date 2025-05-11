import React from "react";
import Icon from "@ant-design/icons";
import {StyleUtils} from "@src/style-utils.ts";

export const AboutIcon01: React.FC<{ width: number, height: number, color: string }> = (props) => {
    const aboutIcon = () => (
        <svg  className="about-icon-01" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
              width={`${StyleUtils.numberToPixels(props.width)}`} height={`${StyleUtils.numberToPixels(props.height)}`}>
            <path
                d="M107.793067 892.040533C207.0528 804.078933 170.666667 716.8 170.666667 716.8l238.933333 68.266667s-125.0304 127.010133-287.300267 136.192l-14.506666-29.218134z"
                fill={props.color}></path>
            <path
                d="M119.466667 904.533333m-17.066667 0a17.066667 17.066667 0 1 0 34.133333 0 17.066667 17.066667 0 1 0-34.133333 0Z"
                fill={props.color}></path>
            <path
                d="M512 102.4C266.922667 102.4 68.266667 270.506667 68.266667 477.866667s198.656 375.466667 443.733333 375.466666 443.733333-168.106667 443.733333-375.466666S757.077333 102.4 512 102.4z m34.133333 580.266667h-68.266666v-238.933334h68.266666v238.933334z m-34.133333-324.266667a51.2 51.2 0 1 1 0.034133-102.434133A51.2 51.2 0 0 1 512 358.4z"
                fill={props.color}></path>
        </svg>
    )
    return <Icon component={aboutIcon} {...props}/>
}