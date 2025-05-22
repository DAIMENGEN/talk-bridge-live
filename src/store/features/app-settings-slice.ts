import {createSlice, PayloadAction} from "@reduxjs/toolkit";

export type AppSettingsStore = {
    speechThreshold: number;
    silenceStreakThreshold: number;
    speaker: {
        name?: string;
    }
    microphone: {
        name?: string;
        gain: number;
        isMuted: boolean;
    }
}

const initialState: AppSettingsStore = {
    speechThreshold: 0.75,
    silenceStreakThreshold: 3,
    speaker: {},
    microphone: {
        gain: 1.0,
        isMuted: false
    }
}

const appSettingsSlice = createSlice({
    name: "app-settings",
    initialState,
    reducers: {
        setMicrophoneGain: (state, action: PayloadAction<number>) => {
            state.microphone.gain = action.payload;
        },
        setSpeechThreshold: (state, action: PayloadAction<number>) => {
            state.speechThreshold = action.payload;
        },
        setSilenceStreakThreshold: (state, action: PayloadAction<number>) => {
            state.silenceStreakThreshold = action.payload;
        },
        setSpeakerName: (state, action: PayloadAction<string | undefined>) => {
            state.speaker.name = action.payload;
        },
        setMicrophoneName: (state, action: PayloadAction<string | undefined>) => {
            state.microphone.name = action.payload;
        }
    }
});

export const {
    setSilenceStreakThreshold,
    setMicrophoneGain,
    setSpeechThreshold,
    setMicrophoneName
} = appSettingsSlice.actions;

export default appSettingsSlice.reducer;