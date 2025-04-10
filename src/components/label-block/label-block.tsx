import "./label-block.scss";
import React from "react";


interface LabelBlockProps {
    label: string
    content?: React.ReactNode;
}

export const LabelBlock: React.FC<LabelBlockProps> = ({label, content}) => {
    return (
        <div className={"label-block"}>
            <div className={"label-block-label"}>
                {label}
            </div>
            <div className={"label-block-content"}>
                {content}
            </div>
        </div>
    )
}