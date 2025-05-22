import sessionStorage from "redux-persist/lib/storage/session";
import {persistReducer, persistStore} from "redux-persist";
import {combineReducers, configureStore} from "@reduxjs/toolkit";
import appSettingsReducer from "@src/store/features/app-settings-slice";
import meetingContentReducer from "@src/store/features/meeting-content-slice.ts";
import {TypedUseSelectorHook, useDispatch, useSelector} from "react-redux";

export const persistConfig = {
    key: "root",
    storage: sessionStorage,
    whitelist: ["app-settings-persist", "meeting-content-persist"]
};

export const appSettingsPersistConfig = {
    key: "app-settings-persist",
    storage: sessionStorage
};

export const meetingContentPersistConfig = {
    key: "meeting-content-persist",
    storage: sessionStorage
};

const reducers = combineReducers({
    appSettings: persistReducer(appSettingsPersistConfig, appSettingsReducer),
    meetingContent: persistReducer(meetingContentPersistConfig, meetingContentReducer)
});

const persistedReducer = persistReducer(persistConfig, reducers);

export const appStore = configureStore({
    reducer: persistedReducer,
    middleware: (getDefaultMiddleware) =>
        getDefaultMiddleware({
            serializableCheck: false,
        }),
});

export const persistor = persistStore(appStore);

export type AppDispatch = typeof appStore.dispatch;

export type AppState = ReturnType<typeof appStore.getState>;

export const useAppDispatch = () => useDispatch<AppDispatch>();

export const useAppSelector: TypedUseSelectorHook<AppState> = useSelector;