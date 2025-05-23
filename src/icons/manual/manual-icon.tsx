import React from "react";
import Icon from "@ant-design/icons";
import {StyleUtils} from "@src/style-utils.ts";

export const ManualIcon: React.FC<{ width: number, height: number, color: string }> = (props) => {
    const manualIcon = () => (
        <svg className="manual-icon" viewBox="0 0 1025 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
             width={`${StyleUtils.numberToPixels(props.width)}`} height={`${StyleUtils.numberToPixels(props.height)}`}>
            <path
                d="M236.96 216.416c0 106.048 82.464 192.064 184.224 192.064 101.696 0 184.192-86.016 184.192-192.064 0-106.016-82.496-192-184.192-192-101.792 0-184.224 85.984-184.224 192z m575.648 617.472c-13.92-30.944-42.496-122.048-75.2-122.048-32.64 0-35.296 12.32-35.296 12.32l35.296 94.592-180.736-15.776-76.416-180.736c-74.688-184.224-149.472-204.992-222.432-204.992-35.328 0-109.504 4.224-166.336 76.064C34.784 565.12 0 962.88 0 962.88h346.048L221.696 721.088l52.512-33.184s9.312 20.032 70.272 149.824C405.536 967.616 447.2 985.312 447.2 985.312h325.216c47.84 0 86.464-15.456 86.464-77.312 0-61.824-46.272-74.112-46.272-74.112z m0 0"
                fill={props.color}></path>
            <path
                d="M690.464 474.688s-33.568 64.928-60.16 92.704c-26.688 27.776-78.816 98.464-38.24 155.232L711.392 526.72l249.12 113.632-91.488 136.704s-12.8 54.464 4.608 56.768c17.376 2.304 151.808-216.672 151.808-216.672L690.464 474.688z m0 0"
                fill={props.color}></path>
        </svg>
    )
    return <Icon component={manualIcon} {...props}/>
}