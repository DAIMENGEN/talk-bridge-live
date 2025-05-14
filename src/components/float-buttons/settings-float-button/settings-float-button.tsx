import "./settings-float-button.scss";
import {SettingsIcon} from "@src/icons";
import {FloatButton, Layout} from "antd";
import {useState} from "react";
import {DraggableModal} from "@src/components/common";
import {DARK_MODE_BACKGROUND_COLOR} from "@src/theme/theme.ts";

export const SettingsFloatButton = () => {
    const [openSettings, setOpenSettings] = useState(false);
    return (
        <>
            <FloatButton icon={<SettingsIcon width={20} height={20} color={DARK_MODE_BACKGROUND_COLOR}/>}
                         tooltip={"Settings"}
                         onClick={() => setOpenSettings(true)}
            />
            <DraggableModal closable
                            footer={null}
                            width={"80%"}
                            centered={true}
                            title={"Settings"}
                            open={openSettings}
                            classNames={{
                                wrapper: "setting-draggable-modal",
                            }}
                            maskClosable={false}
                            onCancel={() => setOpenSettings(false)}>
                <Layout>
                    <Layout.Sider>
                        Sider
                    </Layout.Sider>
                    <Layout>
                        <Layout.Header>
                            Header
                        </Layout.Header>
                        <Layout.Content>
                            Content
                        </Layout.Content>
                    </Layout>
                </Layout>
            </DraggableModal>
        </>
    )
}