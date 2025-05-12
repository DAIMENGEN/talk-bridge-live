import "./legal-notice.scss";
import {Typography} from "antd";

export const LegalNotice = () => {
    const {Link} = Typography;
    return (
        <div className={"legal-notice"}>
            <span>
                By using Talk Bridge Live, you agree to our
                <Link href={"#"} target={"_blank"}> Terms of Service </Link>
                and
                <Link href={"#"} target={"_blank"}> Privacy Policy </Link>
                .
            </span>
        </div>
    )
}