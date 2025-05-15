import "./reset.scss";
import App from "./App";
import React from "react";
import init from "src-wasm";
import {ConfigProvider} from "antd";
import {Provider} from "react-redux";
import ReactDOM from "react-dom/client";
import {appStore, persistor} from "@src/store/store.ts";
import {PersistGate} from "redux-persist/integration/react";
import {ADVANTEST_COLOR, DARK_MODE_FONT_COLOR} from "@src/theme/theme.ts";

init().then(() => {
    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <React.StrictMode>
            <Provider store={appStore}>
                <PersistGate persistor={persistor}>
                    <ConfigProvider theme={{
                        token: {
                            colorPrimary: ADVANTEST_COLOR
                        },
                        components: {
                            Select: {
                                optionActiveBg: "#7a7a7a",
                                optionSelectedBg: "#2a2a2a",
                                optionSelectedColor: DARK_MODE_FONT_COLOR,
                            }
                        }
                    }}>
                        <App/>
                    </ConfigProvider>
                </PersistGate>
            </Provider>
        </React.StrictMode>,
    )
})