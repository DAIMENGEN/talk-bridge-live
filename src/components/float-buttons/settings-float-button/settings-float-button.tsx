import "./settings-float-button.scss";
import {AudioIcon, PrivacyIcon, SettingsIcon} from "@src/icons";
import {FloatButton, Layout, Menu} from "antd";
import {useState} from "react";
import {DraggableModal} from "@src/components/common";
import {DARK_MODE_BACKGROUND_COLOR} from "@src/theme/theme.ts";
import {MenuItem} from "@src/types.ts";

export const SettingsFloatButton = () => {
    const items: MenuItem[] = [
        {
            key: '1',
            icon: <SettingsIcon width={20} height={20} color={"#D0D4D9"}/>,
            label: "General",
        },
        {
            key: '2',
            icon: <AudioIcon width={20} height={20} color={"#D0D4D9"}/>,
            label: "Audio",
        },
        {
            key: '3',
            icon: <PrivacyIcon width={20} height={20} color={"#D0D4D9"}/>,
            label: "Privacy",
        },
    ];
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
                        <Menu mode="inline" theme={"dark"} items={items}/>
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