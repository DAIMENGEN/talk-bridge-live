import "./report-issue-float-button.scss";
import {FloatButton} from "antd";
import {IssueIcon} from "@src/icons";
import {DARK_MODE_BACKGROUND_COLOR} from "@src/theme/theme.ts";

export const ReportIssueFloatButton = () => {
    return (
        <FloatButton icon={<IssueIcon width={20} height={20} color={DARK_MODE_BACKGROUND_COLOR}/>}
                     tooltip={"Report an issue"}
        />
    )
}