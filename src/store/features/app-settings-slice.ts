import {createSlice, PayloadAction} from "@reduxjs/toolkit";

export type AppSettingsStore = {
    isOpenSettings: boolean;
    isShowSettingsButton: boolean;
    speakerVolume: number;
    microphoneGain: number;
    selectedSpeakerName?: string;
    selectedMicrophoneName?: string;
}

const initialState: AppSettingsStore = {
    isOpenSettings: false,
    isShowSettingsButton: false,
    speakerVolume: 1,
    microphoneGain: 1,
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
        setSpeakerVolume: (state, action: PayloadAction<number>) => {
            state.speakerVolume = action.payload;
        },
        setMicrophoneGain: (state, action: PayloadAction<number>) => {
            state.microphoneGain = action.payload;
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
    setSpeakerVolume,
    setMicrophoneGain,
    setSelectedSpeakerName,
    setSelectedMicrophoneName
} = appSettingsSlice.actions;

export default appSettingsSlice.reducer;