import "./voice-activity-indicator.scss";
import React from "react";
import {Flex} from "antd";

export const VoiceActivityIndicator: React.FC<{ probability: number }> = ({probability}) => {
    return (
        <Flex justify={"space-between"} className={"voice-activity-indicator"}>
            {Array.from({length: 100}).map((_, index) => {
                let className = "bar";
                if (index < Math.round(probability * 100)) {
                    if (index < 50) {
                        className += " presence-bar";
                    } else {
                        className += " nominal-bar";
                    }
                }
                return <div key={index} className={className}/>;
            })}
        </Flex>
    )
}