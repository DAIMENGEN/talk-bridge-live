use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptData {
    datetime: String,
    transcript: String,
}

impl TranscriptData {
    pub fn new(datetime: String, transcript: String) -> Self {
        Self {
            datetime,
            transcript,
        }
    }
}
