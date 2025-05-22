import {createSlice, PayloadAction} from "@reduxjs/toolkit";

export type Utterance = {
    speaker: string;
    datetime: string;
    speechText: string;
    speechData: number[];
    translatedText: {
        [language: string]: string;
    };
}

export type MeetingContentStore = {
    utterances: Utterance[];
}

const initialState: MeetingContentStore = {
    utterances: []
};

const meetingContentSlice = createSlice({
    name: "meeting-content",
    initialState,
    reducers: {
        appendUtterance: (state, action: PayloadAction<Utterance>) => {
            state.utterances.push(action.payload);
        },
        clearUtterances: (state) => {
            state.utterances.splice(0, state.utterances.length);
        }
    }
});

export const { appendUtterance, clearUtterances } = meetingContentSlice.actions;

export default meetingContentSlice.reducer;