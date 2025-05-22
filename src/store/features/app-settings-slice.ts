import {createSlice, PayloadAction} from "@reduxjs/toolkit";

export type AppSettingsStore = {
    microphoneGain: number;
    speechThreshold: number;
    silenceStreakThreshold: number;
    speakerName?: string;
    microphoneName?: string;
}

const initialState: AppSettingsStore = {
    microphoneGain: 1.0,
    speechThreshold: 0.75,
    silenceStreakThreshold: 3,
    speakerName: undefined,
    microphoneName: undefined
}

const appSettingsSlice = createSlice({
    name: "app-settings",
    initialState,
    reducers: {
        setMicrophoneGain: (state, action: PayloadAction<number>) => {
            state.microphoneGain = action.payload;
        },
        setSpeechThreshold: (state, action: PayloadAction<number>) => {
            state.speechThreshold = action.payload;
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
    setSilenceStreakThreshold,
    setMicrophoneGain,
    setSpeechThreshold,
    setSpeakerName,
    setMicrophoneName
} = appSettingsSlice.actions;

export default appSettingsSlice.reducer;