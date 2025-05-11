import {createSlice, PayloadAction} from "@reduxjs/toolkit";

export type AppSettingsStore = {
    isOpenSettings: boolean;
    isShowSettingsButton: boolean;
    microphoneGain: number;
    speechThreshold: number;
    audioGapThreshold: number;
    silenceStreakCount: number;
    selectedSpeakerName?: string;
    selectedMicrophoneName?: string;
}

const initialState: AppSettingsStore = {
    isOpenSettings: false,
    isShowSettingsButton: false,
    microphoneGain: 1.0,
    speechThreshold: 0.75,
    audioGapThreshold: 0.5,
    silenceStreakCount: 3,
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
        setMicrophoneGain: (state, action: PayloadAction<number>) => {
            state.microphoneGain = action.payload;
        },
        setSpeechThreshold: (state, action: PayloadAction<number>) => {
            state.speechThreshold = action.payload;
        },
        setAudioGapThreshold: (state, action: PayloadAction<number>) => {
            state.audioGapThreshold = action.payload;
        },
        setSilenceStreakCount: (state, action: PayloadAction<number>) => {
            state.silenceStreakCount = action.payload;
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
    setSilenceStreakCount,
    setMicrophoneGain,
    setSpeechThreshold,
    setSelectedSpeakerName,
    setAudioGapThreshold,
    setSelectedMicrophoneName
} = appSettingsSlice.actions;

export default appSettingsSlice.reducer;