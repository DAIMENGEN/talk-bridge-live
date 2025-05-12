import React from "react";
import {StyleUtils} from "@src/style-utils";
import Icon from "@ant-design/icons";

export const CloseIcon: React.FC<{ width: number, height: number, color: string }> = (props) => {
    const icon = () => (
        <svg className="close-icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
             width={`${StyleUtils.numberToPixels(props.width)}`} height={`${StyleUtils.numberToPixels(props.height)}`}>
            <path
                d="M509.144072 0a58.50561 58.50561 0 0 1 58.50561 58.50561v438.792076a58.50561 58.50561 0 1 1-117.01122 0V58.50561a58.50561 58.50561 0 0 1 58.50561-58.50561zM207.138113 189.499671a58.096071 58.096071 0 0 1 82.463657 6.962168 59.558711 59.558711 0 0 1-6.932915 83.370494 351.033661 351.033661 0 1 0 455.875715 2.486489 59.558711 59.558711 0 0 1-6.026078-83.370495 58.096071 58.096071 0 0 1 82.522163-6.084583A470.809271 470.809271 0 1 1 41.099191 550.83032a474.012454 474.012454 0 0 1 166.038922-361.272143z"
                fill={props.color}></path>
        </svg>
    )
    return <Icon component={icon} {...props}/>
}