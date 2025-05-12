import "./report-issue-float-button.scss";
import {FloatButton} from "antd";
import {IssueIcon} from "@src/icons";

export const ReportIssueFloatButton = () => {
    return (
        <FloatButton icon={<IssueIcon width={20} height={20} color={"#141414"}/>}
                     tooltip={"Report an issue"}
        />
    )
}