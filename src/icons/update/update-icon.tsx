import React from "react";
import Icon from "@ant-design/icons";
import {StyleUtils} from "@src/style-utils";

export const UpdateIcon: React.FC<{ width: number, height: number, color: string }> = (props) => {
    const updateIcon = () => (
        <svg className="update-icon" viewBox="0 0 1365 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
             width={`${StyleUtils.numberToPixels(props.width)}`} height={`${StyleUtils.numberToPixels(props.height)}`}>
            <path
                d="M675.07687393 38.76937914a379.37321408 379.37321408 0 0 0-374.6409088 319.66728439A316.27579858 316.27579858 0 0 0 359.35317743 985.23062086H596.20510318V748.61531043H438.46156348l236.61531045-315.48708028 236.61531043 315.48708028H753.94864376v236.61531042h236.61531042c196.23296381 0 299.08175238-124.9328839 314.1462608-287.01437154a316.27579858 316.27579858 0 0 0-255.07130464-339.77958578A379.37321408 379.37321408 0 0 0 674.99800154 38.76937914z"
                fill={props.color}></path>
        </svg>
    )
    return <Icon component={updateIcon} {...props}/>
}