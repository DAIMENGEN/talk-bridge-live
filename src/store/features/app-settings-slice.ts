import {createSlice, PayloadAction} from "@reduxjs/toolkit";

export type AppSettingsStore = {
    isOpenSettings: boolean;
    isShowSettingsButton: boolean;
    microphoneGain: number;
    speechThreshold: number;
    audioGapThreshold: number;
    silenceStreakThreshold: number;
    speakerName?: string;
    microphoneName?: string;
}

const initialState: AppSettingsStore = {
    isOpenSettings: false,
    isShowSettingsButton: false,
    microphoneGain: 1.0,
    speechThreshold: 0.75,
    audioGapThreshold: 0.5,
    silenceStreakThreshold: 3,
    speakerName: undefined,
    microphoneName: undefined
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
        setSilenceStreakThreshold: (state, action: PayloadAction<number>) => {
            state.silenceStreakThreshold = action.payload;
        },
        setSpeakerName: (state, action: PayloadAction<string | undefined>) => {
            state.speakerName = action.payload;
        },
        setMicrophoneName: (state, action: PayloadAction<string | undefined>) => {
            state.microphoneName = action.payload;
        }
    }
});

export const {
    setIsOpenSettings,
    setIsShowSettingsButton,
    setSilenceStreakThreshold,
    setMicrophoneGain,
    setSpeechThreshold,
    setSpeakerName,
    setAudioGapThreshold,
    setMicrophoneName
} = appSettingsSlice.actions;

export default appSettingsSlice.reducer;