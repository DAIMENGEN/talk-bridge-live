import {createSlice, PayloadAction} from "@reduxjs/toolkit";

export type AppSettingsStore = {
    isOpenSettings: boolean;
    isShowSettingsButton: boolean;
    audioTolerance: number;
    microphoneGain: number;
    speechThreshold: number;
    selectedSpeakerName?: string;
    selectedMicrophoneName?: string;
}

const initialState: AppSettingsStore = {
    isOpenSettings: false,
    isShowSettingsButton: false,
    audioTolerance: 1,
    microphoneGain: 1.0,
    speechThreshold: 0.5,
    selectedSpeakerName: undefined,
    selectedMicrophoneName: undefined
}

const appSettingsSlice = createSlice({
    name: "appSettings",
    initialState,
    reducers: {
        setIsOpenSettings: (state, action: PayloadAction<boolean>) => {
            state.isOpenSettings = action.payload;
        },
        setIsShowSettingsButton: (state, action: PayloadAction<boolean>) => {
            state.isShowSettingsButton = action.payload;
        },
        setAudioTolerance: (state, action: PayloadAction<number>) => {
            state.audioTolerance = action.payload;
        },
        setMicrophoneGain: (state, action: PayloadAction<number>) => {
            state.microphoneGain = action.payload;
        },
        setSpeechThreshold: (state, action: PayloadAction<number>) => {
            state.speechThreshold = action.payload;
        },
        setSelectedSpeakerName: (state, action: PayloadAction<string | undefined>) => {
            state.selectedSpeakerName = action.payload;
        },
        setSelectedMicrophoneName: (state, action: PayloadAction<string | undefined>) => {
            state.selectedMicrophoneName = action.payload;
        }
    }
});

export const {
    setIsOpenSettings,
    setIsShowSettingsButton,
    setAudioTolerance,
    setMicrophoneGain,
    setSpeechThreshold,
    setSelectedSpeakerName,
    setSelectedMicrophoneName
} = appSettingsSlice.actions;

export default appSettingsSlice.reducer;