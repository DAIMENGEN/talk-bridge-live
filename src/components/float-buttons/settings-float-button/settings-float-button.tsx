import "./settings-float-button.scss";
import {AudioIcon, PrivacyIcon, SettingsIcon} from "@src/icons";
import {FloatButton, Layout, Menu} from "antd";
import {useCallback, useState} from "react";
import {DraggableModal} from "@src/components/common";
import {DARK_MODE_BACKGROUND_COLOR, DARK_MODE_FONT_COLOR} from "@src/theme/theme.ts";
import {MenuItem} from "@src/types.ts";

export const SettingsFloatButton = () => {
    const items: MenuItem[] = [
        {
            key: "1",
            icon: <SettingsIcon width={20} height={20} color={DARK_MODE_FONT_COLOR}/>,
            label: "General",
        },
        {
            key: "2",
            icon: <AudioIcon width={20} height={20} color={DARK_MODE_FONT_COLOR}/>,
            label: "Audio",
        },
        {
            key: "3",
            icon: <PrivacyIcon width={20} height={20} color={DARK_MODE_FONT_COLOR}/>,
            label: "Privacy",
        },
    ];
    const [openSettings, setOpenSettings] = useState(false);
    const [selectedKey, setSelectedKey] = useState<string>("1");
    const renderSettingsComponent = useCallback(() => {
        switch (selectedKey) {
            case "1":
                return <GeneralSettings />;
            case "2":
                return <AudioSettings />;
            case "3":
                return <PrivacySettings />;
            default:
                return <GeneralSettings />;
        }
    }, [selectedKey]);
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
                            destroyOnClose={true}
                            classNames={{
                                wrapper: "setting-draggable-modal",
                            }}
                            maskClosable={false}
                            onCancel={() => {
                                setSelectedKey("1");
                                setOpenSettings(false);
                            }}>
                <Layout>
                    <Layout.Sider>
                        <Menu defaultSelectedKeys={[selectedKey]} mode="inline" theme={"dark"} items={items} onSelect={({key}) => setSelectedKey(key)}/>
                    </Layout.Sider>
                    <Layout>
                        {renderSettingsComponent()}
                    </Layout>
                </Layout>
            </DraggableModal>
        </>
    )
}

export const GeneralSettings = () => {
    return (
        <>
            <Layout.Header>
                General
            </Layout.Header>
            <Layout.Content>
                Content
            </Layout.Content>
        </>
    )
}

export const AudioSettings = () => {
    return (
        <>
            <Layout.Header>
                Audio
            </Layout.Header>
            <Layout.Content>
                Content
            </Layout.Content>
        </>
    )
}

export const PrivacySettings = () => {
    return (
        <>
            <Layout.Header>
                Privacy
            </Layout.Header>
            <Layout.Content>
                Content
            </Layout.Content>
        </>
    )
}