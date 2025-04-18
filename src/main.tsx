import "./reset.scss";
import App from "./App";
import React from "react";
import init from "src-wasm";
import {ConfigProvider} from "antd";
import {Provider} from "react-redux";
import ReactDOM from "react-dom/client";
import {appStore, persistor} from "@src/store/store.ts";
import {PersistGate} from "redux-persist/integration/react";

init().then(() => {
    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <React.StrictMode>
            <Provider store={appStore}>
                <PersistGate persistor={persistor}>
                    <ConfigProvider theme={{
                        token: {
                            colorPrimary: "#91003c"
                        }
                    }}>
                        <App/>
                    </ConfigProvider>
                </PersistGate>
            </Provider>
        </React.StrictMode>,
    )
})